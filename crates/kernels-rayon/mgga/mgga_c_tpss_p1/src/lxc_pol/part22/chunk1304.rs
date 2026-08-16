//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1304/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1304(t61079: f64, t18751: f64, t219: f64, t1811: f64, t31814: f64, t18802: f64, t2436: f64, t5848: f64, t8096: f64, t61868: f64, t18999: f64, t508: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t62711 = 455.0_f64 / 648.0_f64 * t61079;
    let t62731 = t18751 * t219;
    let t62807 = t1811 * t31814;
    let t62820 = t18802 * t2436;
    let t62829 = t5848 * t8096;
    let t63006 = 308.0_f64 / 27.0_f64 * t61868;
    let t63101 = t508 * t18999;
    (t62711, t62731, t62807, t62820, t62829, t63006, t63101)
}
