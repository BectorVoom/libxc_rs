//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 763/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk763(t32111: f64, t32364: f64, t103: f64, t1337: f64, t5618: f64, t28: f64, t497: f64, t7212: f64, t32325: f64, t369: f64, t108: f64, t432: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32365 = t32111 + t32364;
    let t32366 = t32365 * t103;
    let t32370 = t5618 * t1337;
    let t32371 = t28 * t32370;
    let t32374 = t7212 * t497;
    let t32375 = t28 * t32374;
    let t32378 = t369 * t32325;
    let t32379 = t32378 * t108;
    let t32380 = t28 * t32379;
    let t32385 = t1337 * t432;
    (t32365, t32366, t32370, t32371, t32374, t32375, t32378, t32379, t32380, t32385)
}
