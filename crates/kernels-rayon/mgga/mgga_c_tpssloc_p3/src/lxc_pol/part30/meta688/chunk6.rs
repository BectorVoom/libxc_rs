//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2191/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2191(t28821: f64, t7000: f64, t1983: f64, t24990: f64, t26167: f64, t7687: f64, t91620: f64, t28002: f64, t6535: f64, t12725: f64, t7461: f64, t19456: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97836 = t28821 * t7000;
    let t97839 = 6.0_f64 * t1983 * t26167 * t24990;
    let t97842 = 6.0_f64 * t1983 * t91620 * t7687;
    let t97844 = 4.0_f64 * t28002 * t6535;
    let t97846 = 4.0_f64 * t12725 * t7461;
    let t97848 = 4.0_f64 * t19456 * t7461;
    (t97836, t97839, t97842, t97844, t97846, t97848)
}
