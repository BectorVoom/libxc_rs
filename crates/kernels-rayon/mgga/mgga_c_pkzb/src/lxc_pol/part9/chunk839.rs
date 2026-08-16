//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 839/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk839(t12: f64, t6069: f64, t6070: f64, t5100: f64, t2159: f64, t2163: f64, t318: f64, t319: f64, t808: f64, t810: f64, t201: f64, t199: f64, t204: f64, t334: f64, t3981: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t84 = t12 <= zeta_threshold;
    let t203 = rho0 <= dens_threshold || t84;
    let t6071 = t6069 + t6070;
    let t6078 = piecewise3(t84, 0.0_f64, t5100);
    let t6082 = piecewise3(t203, 0.0_f64, t6071 * t319 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2159 * t810 + 3.0_f64 / 2.0_f64 * t808 * t2163 + t318 * t6078 / 2.0_f64);
    let t6083 = t201 * t6082;
    let t6084 = t199 * t6083;
    let t6085 = 0.2390625e-1_f64 * t6084;
    let t6087 = t204 * t3981 * t334;
    (t6071, t6078, t6085, t6087)
}
