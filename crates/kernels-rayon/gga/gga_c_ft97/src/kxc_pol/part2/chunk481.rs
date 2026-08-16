//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 481/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk481(t2666: f64, t2771: f64, t2498: f64, t848: f64, t2502: f64, t2: f64, t2680: f64, t192: f64, t2682: f64, t2739: f64, t852: f64, t2761: f64, t2762: f64, t2764: f64, t2767: f64, t462: f64, t92: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2772 = t2771 * t2666;
    let t2775 = t848 * t2498;
    let t2778 = t848 * t2502;
    let t2781 = t2680 * t2;
    let t2783 = t192 * t2781 * t2682;
    let t2787 = t192 * t852 * t2739;
    let t2789 = t2761 + 2.0_f64 / 9.0_f64 * t2762 + 2.0_f64 / 3.0_f64 * t2764 - 2.0_f64 / 9.0_f64 * t462 * t2767 + 2.0_f64 / 3.0_f64 * t462 * t2772 + 2.0_f64 / 3.0_f64 * t462 * t2775 - t462 * t2778 / 3.0_f64 + 2.0_f64 * t92 * t2783 - t92 * t2787;
    (t2772, t2775, t2778, t2781, t2783, t2787, t2789)
}
