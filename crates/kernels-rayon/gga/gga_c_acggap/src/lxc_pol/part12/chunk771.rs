//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 771/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk771(t7934: f64, t8306: f64, t4210: f64, t7942: f64, t7965: f64, t7963: f64, t119: f64, t2217: f64, t2219: f64, t310: f64, t150: f64, t187: f64, t8301: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8307 = t8306 * t7934;
    let t8310 = t8306 * t4210;
    let t8311 = t7942 * t8310;
    let t8313 = t8306 * t7965;
    let t8314 = t7963 * t8313;
    let t8316 = t119 * t2217;
    let t8319 = t310 * t2219;
    let t8322 = t8301 * t150 * t187;
    (t8307, t8310, t8311, t8313, t8314, t8316, t8319, t8322)
}
