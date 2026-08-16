//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 791/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk791(t614: f64, t7313: f64, t28: f64, t376: f64, t7314: f64, t1349: f64, t2035: f64, t538: f64, t7318: f64, t554: f64, t23823: f64, t40: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32742 = t7313 * t614;
    let t32743 = t28 * t32742;
    let t32748 = t376 * t7314;
    let t32750 = t1349 * t32748 / 9.0_f64;
    let t32752 = t2035 * t7318 * t538;
    let t32756 = t2035 * t7318 * t554;
    let t32763 = t23823 * t40;
    (t32742, t32743, t32748, t32750, t32752, t32756, t32763)
}
