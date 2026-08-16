//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 749/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk749(t26361: f64, t225: f64, t7919: f64, t1824: f64, t2085: f64, t26393: f64, t26406: f64, t26429: f64, t1338: f64, t7918: f64, t26127: f64, t111: f64, t7786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27067 = 0.38381794893125283518e-1_f64 * t26361;
    let t27068 = t7919 * t225;
    let t27074 = t2085 * t1824;
    let t27082 = 0.16449340668482264365e-1_f64 * t26393;
    let t27088 = 0.38381794893125283518e-1_f64 * t26406;
    let t27096 = 0.38381794893125283518e-1_f64 * t26429;
    let t27097 = t1338 * t7918;
    let t27166 = 2.0_f64 / 3.0_f64 * t26127;
    let t27188 = t7786 * t111;
    (t27067, t27068, t27074, t27082, t27088, t27096, t27097, t27166, t27188)
}
