//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1905/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1905(t13826: f64, t7271: f64, t13923: f64, t7264: f64, t14036: f64, t25997: f64, t13946: f64, t26028: f64, t13941: f64, t94423: f64, t14005: f64, t13834: f64, t27940: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t98176 = t7271 * t13826;
    let t98178 = t7264 * t13923;
    let t98180 = t25997 * t14036;
    let t98182 = t26028 * t13946;
    let t98185 = t94423 * t13941;
    let t98187 = t94423 * t14005;
    let t98189 = t27940 * t13834;
    (t98176, t98178, t98180, t98182, t98185, t98187, t98189)
}
