//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 779/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk779<F: Float>(t718: F, t7336: F, t1934: F, t2532: F, t2585: F, t740: F, t1872: F, t2558: F, t5060: F, t1849: F, t2505: F, t140: F, t2554: F, t430: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t17936 = t7336 * t718;
    let t17969 = t1934 * t2532;
    let t17975 = t740 * t2585;
    let t17976 = t1872 * t17975;
    let t17982 = t2558 * t5060;
    let t17983 = t17982 * sigma2;
    let t17991 = t2505 * t1849;
    let t18005 = t140 * t430 * t2554;
    (t17936, t17969, t17976, t17983, t17991, t18005)
}
