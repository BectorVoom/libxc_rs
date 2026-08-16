//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3880/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3880(t124: f64, t1370: f64, t47199: f64, t47216: f64, t47229: f64, t48945: f64, t48947: f64, t48951: f64, t48955: f64, t48971: f64, t48975: f64, t73578: f64, t74547: f64, t800: f64) -> f64 {
    let t74558 = 0.50820002809285328224e-4_f64 * t48945 + 0.30234122406223992295e0_f64 * t48947 - 0.57165357490759649296e-3_f64 * t48951 - 0.28582678745379824648e-3_f64 * t48955 + 7.0_f64 / 72.0_f64 * t74547 - t1370 * t800 * t124 * t73578 / 48.0_f64 - 0.25692334753583138158e-2_f64 * t47199 - 0.27104001498285508386e-3_f64 * t47216 - 0.56688979511669985553e-2_f64 * t47229 - 0.16006300097412701803e-1_f64 * t48971 - 0.50820002809285328224e-4_f64 * t48975;
    t74558
}
