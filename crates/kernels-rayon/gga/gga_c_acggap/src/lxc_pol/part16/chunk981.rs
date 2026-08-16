//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 981/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk981(t137: f64, t3300: f64, t30407: f64, t31097: f64, t495: f64, t7325: f64, t30543: f64, t8610: f64, t30934: f64, t8614: f64, t7433: f64, t8522: f64) -> (f64, f64, f64, f64, f64) {
    let t34692 = t3300 * t137;
    let t34698 = t30407 * t31097 * t7325 * t495;
    let t34702 = t30543 * t8610;
    let t34703 = 0.12862205435420921092e-1_f64 * t34702;
    let t34704 = t30934 * t8614;
    let t34710 = t7433 * t8522;
    (t34692, t34698, t34703, t34704, t34710)
}
