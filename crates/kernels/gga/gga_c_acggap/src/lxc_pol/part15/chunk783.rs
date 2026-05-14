//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 783/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk783<F: Float>(t435: F, t507: F, t1449: F, t322: F, t1460: F, t1444: F, t1421: F, t301: F, t1439: F, t1742: F, t1416: F, t1772: F, t1410: F, t1539: F, t1345: F, t145: F, t5784: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t20559 = t507 * t435;
    let t20992 = t1449 * t322;
    let t21118 = t1460 * t322;
    let t21143 = t1444 * t322;
    let t21955 = t1421 * t301;
    let t22040 = t1439 * t322;
    let t22099 = t1742 * t435;
    let t22275 = t1416 * t322;
    let t22705 = t435 * t1772;
    let t23688 = t1539 * t1410;
    let t23745 = t1345 * t322;
    let t24196 = t5784 * t145;
    (t20559, t20992, t21118, t21143, t21955, t22040, t22099, t22275, t22705, t23688, t23745, t24196)
}
