//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 854/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk854<F: Float>(t1882: F, t4280: F, t10443: F, t4146: F, t10533: F, t10539: F, t10545: F, t10670: F, t10678: F, t10693: F, t15309: F, t15314: F, t15318: F, t15322: F, t15325: F, t15329: F, t15334: F, t1901: F, t446: F) -> (F,) {
    let t15336 = 2.0 / 9.0 * t1882 * t4280;
    let t15338 = t10443 * t4146;
    let t15341 = -2.0 / 9.0 * t1901 * t15309 - 4.0 / 9.0 * t1901 * t15314 - 2.0 / 9.0 * t10533 - 4.0 / 81.0 * t15318 + 2.0 / 27.0 * t10539 - 2.0 / 27.0 * t10545 - 2.0 / 3.0 * t446 * t15322 - 2.0 / 3.0 * t446 * t15325 + 4.0 / 27.0 * t15329 + 2.0 / 81.0 * t10670 + t10678 / 27.0 + t15334 + t15336 + 2.0 / 9.0 * t10693 + 2.0 / 9.0 * t1901 * t15338;
    (t15341,)
}
