//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 905/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk905(t1268: f64, t2181: f64, t2183: f64, t4028: f64, t652: f64, t7458: f64, t7676: f64, t8221: f64, t8231: f64, t8235: f64, t8237: f64) -> f64 {
    let t8240 = 2.0_f64 * t1268 * t8235 + 2.0_f64 * t1268 * t8237 - 2.0_f64 * t2181 * t4028 - 2.0_f64 * t2181 * t7458 + 2.0_f64 * t2183 * t4028 + 2.0_f64 * t2183 * t7676 - 2.0_f64 * t652 * t8221 - 2.0_f64 * t652 * t8231;
    t8240
}
