//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 247/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk247(t225: f64, t680: f64, t705: f64, t710: f64, t719: f64, t752: f64, t755: f64, t760: f64, t765: f64, t244: f64, t68: f64, t776: f64) -> (f64, f64, f64) {
    let t822 = (t680 + t705 + t710 + t719 + t752 + t755 - t760 - t765) * t225;
    let t824 = t68 * t244;
    let t825 = t824 * t776;
    (t822, t824, t825)
}
