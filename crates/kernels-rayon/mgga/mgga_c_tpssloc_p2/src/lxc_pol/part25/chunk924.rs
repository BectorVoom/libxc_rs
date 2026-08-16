//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 924/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk924(t11896: f64, t1246: f64, t11707: f64, t3609: f64, t3623: f64, t3620: f64, t5079: f64, t10471: f64, t1209: f64, t11712: f64, t475: f64, t6739: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11897 = t11896 * t1246;
    let t11904 = t11707 * t3609;
    let t11907 = t11707 * t3623;
    let t11910 = t3620 * t5079;
    let t11913 = t10471 * t1209;
    let t11914 = t11712 * t11913;
    let t11915 = t6739 * t475;
    (t11897, t11904, t11907, t11910, t11914, t11915)
}
