//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 677/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk677(t1882: f64, t2667: f64, t2336: f64, t2671: f64, t89: f64, t2680: f64, t683: f64, t191: f64, t7640: f64, t2683: f64, t375: f64, t793: f64, t9733: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10243 = t1882 * t2667;
    let t10246 = t89 * t2336 * t2671;
    let t10248 = t683 * t2680;
    let t10261 = t191 * t7640;
    let t10276 = t89 * t375 * t2683;
    let t10279 = t89 * t9733 * t793;
    (t10243, t10246, t10248, t10261, t10276, t10279)
}
