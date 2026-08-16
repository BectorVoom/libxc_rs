//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 744/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk744(t1307: f64, t6968: f64, t6637: f64, t6888: f64, t2009: f64, t794: f64, t6897: f64, t1338: f64, t6604: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6969 = t6968 * t1307;
    let t6970 = t6637 * t6969;
    let t6971 = t6888 * t6970;
    let t6973 = t794 * t2009;
    let t6974 = t6897 * t6973;
    let t6975 = 0.41123351671205660912e-2_f64 * t6974;
    let t6976 = t6604 * t1338;
    (t6969, t6970, t6971, t6973, t6975, t6976)
}
