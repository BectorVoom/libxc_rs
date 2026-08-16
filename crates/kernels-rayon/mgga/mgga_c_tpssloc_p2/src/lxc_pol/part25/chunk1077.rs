//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1077/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1077(t2678: f64, t852: f64, t225: f64, t9520: f64, t112: f64, t12512: f64, t111: f64, t3931: f64, t2311: f64, t671: f64, t2363: f64, t649: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40955 = t852 * t2678;
    let t41554 = t9520 * t225;
    let t45557 = t12512 * t112;
    let t45560 = t3931 * t111;
    let t45602 = t2311 * t671;
    let t45637 = t649 * t2363;
    (t40955, t41554, t45557, t45560, t45602, t45637)
}
