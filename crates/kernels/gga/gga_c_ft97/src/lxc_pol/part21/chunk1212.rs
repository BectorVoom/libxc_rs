//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1212/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1212<F: Float>(t1882: F, t29862: F, t29926: F, t103510: F, t103686: F, t103753: F, t103761: F, t116260: F, t11863: F, t15951: F, t15955: F, t15959: F, t16035: F, t16305: F, t16313: F, t1901: F, t23323: F, t25924: F, t25929: F, t25955: F, t26318: F, t3113: F, t47443: F, t47659: F, t47666: F, t60805: F, t61053: F, t8557: F, t91739: F, t92072: F) -> (F,) {
    let t117968 = t1882 * t29862;
    let t117987 = t1882 * t29926;
    let t117992 = -4.0 / 27.0 * t47666 * t103510 * t16313 + 4.0 / 3.0 * t47659 * t103761 * t15959 + 4.0 / 9.0 * t47659 * t91739 * t16305 + 8.0 / 9.0 * t47659 * t103753 * t15951 - 8.0 / 27.0 * t47666 * t103753 * t15955 + 2.0 / 9.0 * t117968 - 4.0 / 9.0 * t1901 * t11863 * t116260 - 2.0 / 9.0 * t1901 * t8557 * t25955 * t3113 - 2.0 / 9.0 * t1901 * t47443 * t26318 - 4.0 / 9.0 * t1901 * t60805 * t25924 + 4.0 / 27.0 * t1901 * t61053 * t25929 + 8.0 / 27.0 * t92072 + t117987 / 9.0 - t103686 - 2.0 / 9.0 * t1901 * t23323 * t16035;
    (t117992,)
}
