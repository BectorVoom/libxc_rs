//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1245/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1245(t2508: f64, t29194: f64, t2936: f64, t1890: f64, t21446: f64, t9014: f64, t32435: f64, t688: f64, t779: f64, t10816: f64, t32163: f64, t5836: f64) -> (f64, f64, f64, f64) {
    let t32653 = 0.10766736722199113997e0_f64 * t2508 * t2936 * t29194;
    let t32657 = 0.1845726295234133828e0_f64 * t2508 * t9014 * t1890 * t21446;
    let t32658 = t32435 * t688;
    let t32661 = 0.15381052460284448567e-1_f64 * t2508 * t779 * t32658;
    let t32664 = 0.10766736722199113997e0_f64 * t32163 * t10816 * t5836;
    (t32653, t32657, t32661, t32664)
}
