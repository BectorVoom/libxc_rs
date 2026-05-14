//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1180/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1180<F: Float>(t358: F, t6557: F, t102465: F, t102467: F, t102469: F, t102471: F, t103491: F, t11593: F, t116418: F, t16060: F, t16065: F, t16177: F, t16182: F, t1901: F, t22993: F, t23265: F, t26445: F, t3204: F, t3214: F, t3219: F, t4611: F, t46874: F, t47443: F, t5691: F, t60426: F, t6469: F, t8557: F) -> (F, F) {
    let t116817 = t6557 * t358;
    let t116848 = 4.0 * t1901 * t103491 * t6469 * t3214 + 8.0 / 3.0 * t1901 * t60426 * t6469 * t3219 - 2.0 / 9.0 * t1901 * t8557 * t116817 * t3204 - 4.0 / 9.0 * t11593 * t8557 * t23265 * t16065 + t102465 + t102467 + t102469 - t102471 - 2.0 / 9.0 * t1901 * t8557 * t22993 * t4611 - 2.0 / 9.0 * t1901 * t8557 * t5691 * t16177 - 4.0 / 9.0 * t11593 * t8557 * t5691 * t16182 - 2.0 / 9.0 * t1901 * t8557 * t23265 * t16060 + 2.0 / 3.0 * t1901 * t46874 * t116418 - 2.0 / 9.0 * t1901 * t47443 * t26445;
    (t116817, t116848)
}
