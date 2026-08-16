//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 562/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk562(t2010: f64, t6883: f64, t552: f64, t562: f64, t2009: f64, t794: f64, t6897: f64, t1338: f64, t6604: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6966 = t6883 * t2010;
    let t6967 = 0.19190897446562641759e-1_f64 * t6966;
    let t6968 = t552 * t562;
    let t6973 = t794 * t2009;
    let t6974 = t6897 * t6973;
    let t6975 = 0.41123351671205660912e-2_f64 * t6974;
    let t6976 = t6604 * t1338;
    (t6966, t6967, t6968, t6973, t6974, t6975, t6976)
}
