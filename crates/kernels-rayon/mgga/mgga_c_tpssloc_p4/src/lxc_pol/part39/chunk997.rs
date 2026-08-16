//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 997/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk997(t10471: f64, t3502: f64, t11712: f64, t3508: f64, t6739: f64, t11707: f64, t3609: f64, t3623: f64, t1209: f64, t225: f64, t3591: f64, t3482: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11887 = t10471 * t3502;
    let t11888 = t11712 * t11887;
    let t11889 = t6739 * t3508;
    let t11904 = t11707 * t3609;
    let t11907 = t11707 * t3623;
    let t11913 = t10471 * t1209;
    let t11914 = t11712 * t11913;
    let t11925 = t3591 * t225;
    let t11928 = t3482 * t225;
    (t11888, t11889, t11904, t11907, t11914, t11925, t11928)
}
