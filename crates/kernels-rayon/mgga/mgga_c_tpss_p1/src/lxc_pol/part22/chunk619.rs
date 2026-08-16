//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 619/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk619(t2724: f64, t2785: f64, t948: f64, t975: f64, t2703: f64, t366: f64, t2712: f64, t940: f64, t2711: f64, t345: f64, t220: f64, t2768: f64, t2782: f64, t2783: f64, t368: f64, t983: f64, t985: f64) -> (f64, f64, f64, f64) {
    let t2786 = t2785 * t2724;
    let t2790 = t975 * t948;
    let t2794 = t366 * t2703;
    let t2797 = t2712 * t940;
    let t2798 = t2711 * t2797;
    let t2799 = t2785 * t345;
    let t2804 = t220 * t2768 * t368 + 2.0_f64 * t2782 * t2783 * t2786 - t2783 * t2798 * t2799 + 2.0_f64 * t2790 * t983 * t985 + t2794 * t983 * t985;
    (t2786, t2798, t2799, t2804)
}
