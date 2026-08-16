//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1229/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1229(t1011: f64, t6224: f64, t3508: f64, t24661: f64, t475: f64, t24668: f64, t2132: f64, t28525: f64, t1726: f64, t2136: f64, t24659: f64, t27674: f64, t27677: f64, t27681: f64, t27701: f64, t6178: f64, t6184: f64, t6188: f64, t6207: f64, t7310: f64, t7345: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29642 = t6224 * t1011;
    let t29643 = t29642 * t3508;
    let t29644 = t24661 * t29643;
    let t29647 = t29642 * t475;
    let t29648 = t24668 * t29647;
    let t29651 = t2132 * t28525;
    let t29662 = -t7345 * t6207 / 2304.0_f64 - t27677 / 54.0_f64 - 0.16149102437656156342e-2_f64 * t27681 + 0.20186378047070195428e-3_f64 * t27701 + 0.20186378047070195428e-3_f64 * t24659 * t29644 - 0.10093189023535097714e-3_f64 * t24659 * t29648 - 0.10093189023535097714e-3_f64 * t29651 * t2136 + t7310 * t6178 / 216.0_f64 + t27674 * t1726 / 54.0_f64 - t7310 * t6184 / 288.0_f64 - t7310 * t6188 / 144.0_f64;
    (t29643, t29644, t29647, t29648, t29651, t29662)
}
