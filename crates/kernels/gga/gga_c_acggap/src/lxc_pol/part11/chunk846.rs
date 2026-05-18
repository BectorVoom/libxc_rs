//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 846/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk846<F: Float>(t1444: F, t322: F, t506: F, t955: F, t1421: F, t301: F, t1439: F, t157: F, t929: F, t1416: F, t20432: F, t944: F) -> (F, F, F, F, F, F, F, F) {
    let t21143 = t1444 * t322;
    let t21342 = t955 * t506;
    let t21955 = t1421 * t301;
    let t22040 = t1439 * t322;
    let t22048 = t506 * t929 * t157;
    let t22107 = t1416 * t301;
    let t22275 = t1416 * t322;
    let t22401 = t20432 * t944;
    (t21143, t21342, t21955, t22040, t22048, t22107, t22275, t22401)
}
