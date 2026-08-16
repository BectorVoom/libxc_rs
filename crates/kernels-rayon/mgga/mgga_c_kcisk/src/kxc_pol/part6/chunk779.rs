//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 779/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk779(t718: f64, t7336: f64, t1934: f64, t2532: f64, t2585: f64, t740: f64, t1872: f64, t2558: f64, t5060: f64, t1849: f64, t2505: f64, t140: f64, t2554: f64, t430: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64) {
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
