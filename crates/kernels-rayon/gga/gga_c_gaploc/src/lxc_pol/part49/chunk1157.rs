//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1157/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1157(t13918: f64, t7137: f64, t2508: f64, t2580: f64, t47271: f64, t12255: f64, t1897: f64, t7671: f64, t12213: f64, t7068: f64, t13934: f64, t731: f64) -> (f64, f64, f64, f64, f64) {
    let t47640 = t7137 * t13918;
    let t47644 = 0.15381052460284448567e-1_f64 * t2508 * t2580 * t47271;
    let t47646 = t1897 * t12255 * t7671;
    let t47650 = t1897 * t2580 * t12213 * t7068;
    let t47652 = t731 * t13934;
    (t47640, t47644, t47646, t47650, t47652)
}
