//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 684/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk684(t10282: f64, t10286: f64, t10243: f64, t10397: f64, t2832: f64, t870: f64, t1882: f64, t2859: f64, t2854: f64, t192: f64, t7640: f64, t2842: f64, t863: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10641 = t10282 / 9.0_f64;
    let t10643 = 2.0_f64 / 27.0_f64 * t10286;
    let t10649 = 2.0_f64 / 9.0_f64 * t10243;
    let t10658 = 28.0_f64 / 81.0_f64 * t10397;
    let t10666 = t2832 * t870;
    let t10670 = t1882 * t2859;
    let t10678 = t1882 * t2854;
    let t10683 = t192 * t7640;
    let t10688 = t863 * t2842;
    (t10641, t10643, t10649, t10658, t10666, t10670, t10678, t10683, t10688)
}
