//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 529/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk529(t452: f64, t499: f64, t942: f64, t487: f64, t971: f64, t492: f64, t83: f64, t1548: f64, t1551: f64, t1812: f64, t2981: f64, t2986: f64, t2990: f64, t2995: f64, t3003: f64, t3006: f64, t3011: f64, t3016: f64, t3106: f64, t3121: f64, t3159: f64) -> (f64, f64, f64, f64, f64) {
    let t3235 = t452 * t499 * t942;
    let t3238 = t971 * t487;
    let t3239 = t3238 * t492;
    let t3240 = t83 * t3239;
    let t3255 = -t3121 / 4.0_f64 + t3159 / 2.0_f64 + t1812 + t1548 / 9.0_f64 + t1551 / 3.0_f64 + t2981 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t2986 + t2990 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t2995 - 2.0_f64 / 3.0_f64 * t3003 + t3006 / 3.0_f64 + t3011 / 3.0_f64 + 2.0_f64 * t3016 - t3106;
    (t3235, t3238, t3239, t3240, t3255)
}
