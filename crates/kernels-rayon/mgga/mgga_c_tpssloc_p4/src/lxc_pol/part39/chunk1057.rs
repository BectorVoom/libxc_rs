//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1057/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1057(t9924: f64, t9933: f64, t13112: f64, t13114: f64, t13117: f64, t13118: f64, t13121: f64, t13122: f64, t13125: f64, t13129: f64, t13132: f64, t13135: f64, t9853: f64, t9859: f64, t9907: f64, t9921: f64) -> (f64, f64, f64) {
    let t13136 = 8.0_f64 * t9924;
    let t13137 = 12.0_f64 * t9933;
    let t13138 = -t13112 + t9907 - t13114 + t9853 + t13117 + t13118 - t13121 - t9921 - t13122 + t13125 + t13129 + t13132 + t13135 + t13136 + t9859 + t13137;
    (t13136, t13137, t13138)
}
