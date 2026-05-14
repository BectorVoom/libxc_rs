//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1235/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1235<F: Float>(t1882: F, t26147: F, t463: F, t5710: F, t1780: F, t1339: F, t8216: F, t6492: F, t8232: F, t26156: F, t38953: F, t6466: F, t100195: F, t100230: F, t100234: F, t100243: F, t110: F, t11064: F, t11473: F, t11552: F, t11557: F, t11871: F, t1871: F, t1901: F, t26001: F, t446: F, t499: F, t91543: F, t91565: F, t91771: F) -> (F,) {
    let t102664 = 2.0 / 9.0 * t1882 * t26147;
    let t102678 = t463 * t5710;
    let t102682 = t1780 * t5710;
    let t102689 = t8216 * t1339;
    let t102694 = t8232 * t6492;
    let t102697 = 2.0 / 9.0 * t1882 * t26156;
    let t102698 = t38953 * t6466;
    let t102700 = -4.0 / 9.0 * t1901 * t11552 * t100230 + 2.0 / 27.0 * t1901 * t11552 * t100234 - t102664 + 4.0 / 3.0 * t446 * t1871 * t499 * t26001 + 4.0 / 3.0 * t446 * t1871 * t110 * t100195 + 2.0 / 3.0 * t446 * t1871 * t110 * t100243 + 4.0 / 27.0 * t91543 - 4.0 / 9.0 * t1901 * t102678 * t11473 + 4.0 / 27.0 * t1901 * t102682 * t11557 - 2.0 / 9.0 * t1901 * t91771 * t11871 - 4.0 / 9.0 * t1901 * t102689 * t11064 + 2.0 / 27.0 * t91565 - 4.0 / 27.0 * t102694 - t102697 + 4.0 / 81.0 * t102698;
    (t102700,)
}
