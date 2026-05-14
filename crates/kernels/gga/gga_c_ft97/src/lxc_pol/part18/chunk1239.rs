//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1239/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1239<F: Float>(t1332: F, t46556: F, t26379: F, t8392: F, t26383: F, t100072: F, t102436: F, t11623: F, t11810: F, t12046: F, t1780: F, t1901: F, t22862: F, t23249: F, t25924: F, t25929: F, t26113: F, t26318: F, t3195: F, t38711: F, t39107: F, t432: F, t446: F, t452: F, t47410: F, t47759: F, t488: F, t5691: F, t5704: F, t83: F, t91625: F, t91626: F, t979: F) -> (F, F) {
    let t102806 = t46556 * t1332;
    let t102836 = 4.0 / 9.0 * t8392 * t26379;
    let t102838 = 4.0 / 9.0 * t8392 * t26383;
    let t102839 = 2.0 / 9.0 * t1901 * t39107 * t5691 * t12046 - 2.0 / 9.0 * t1901 * t38711 * t26318 - 4.0 / 9.0 * t1901 * t47410 * t25924 + 4.0 / 27.0 * t1901 * t47759 * t25929 - t91625 - t446 * t83 * t102806 / 3.0 + t446 * t452 * t488 * t22862 * t979 / 3.0 - 2.0 * t446 * t83 * t102436 + 4.0 / 3.0 * t446 * t83 * t100072 - 2.0 / 27.0 * t91626 - 4.0 / 27.0 * t1901 * t1780 * t5704 * t3195 + 2.0 / 3.0 * t446 * t452 * t488 * t26113 * t432 + 4.0 / 3.0 * t1901 * t11810 * t23249 * t11623 + t102836 + t102838;
    (t102806, t102839)
}
