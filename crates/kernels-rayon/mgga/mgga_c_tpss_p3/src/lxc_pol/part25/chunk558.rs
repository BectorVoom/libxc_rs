//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 558/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk558(t2724: f64, t2785: f64, t2712: f64, t940: f64, t2711: f64, t345: f64, t375: f64, t1693: f64, t262: f64, t390: f64, t1016: f64, t664: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2786 = t2785 * t2724;
    let t2797 = t2712 * t940;
    let t2798 = t2711 * t2797;
    let t2799 = t2785 * t345;
    let t2813 = t375 * t375;
    let t2814 = 1.0_f64 / t2813;
    let t2834 = t262 * t1693 * t390;
    let t2835 = 0.23744444444444444444e-1_f64 * t2834;
    let t2836 = t664 * t1016;
    (t2786, t2798, t2799, t2813, t2814, t2834, t2835, t2836)
}
