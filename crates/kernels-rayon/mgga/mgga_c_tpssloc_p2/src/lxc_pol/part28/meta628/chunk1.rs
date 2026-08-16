//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1968/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1968(t193: f64, t7125: f64, t26739: f64, t2752: f64, t200: f64, t7109: f64, t24191: f64, t86755: f64, t1877: f64, t2057: f64, t24335: f64, t24339: f64, t24344: f64, t25015: f64, t2522: f64, t25375: f64, t25377: f64, t25381: f64, t25392: f64, t26563: f64, t26756: f64, t6671: f64, t7114: f64, t7475: f64, t86764: f64, t86794: f64, t86806: f64, t86810: f64, t86830: f64, t87957: f64, t87961: f64) -> (f64, f64, f64, f64, f64) {
    let t92271 = t193 * t7125;
    let t92276 = t26739 * t2752;
    let t92295 = t193 * t200 * t7109;
    let t92299 = 6.0_f64 * t24191 * t86755;
    let t92309 = 2.0_f64 * t92271 * t25375 + 2.0_f64 * t26756 * t86794 - t1877 * t92276 * t6671 - t1877 * t24339 * t25392 + 3.0_f64 * t2522 * t2057 * t87957 + 3.0_f64 / 2.0_f64 * t2522 * t2057 * t86764 - t1877 * t7114 * t86806 / 2.0_f64 + 6.0_f64 * t26563 * t86830 - t1877 * t24339 * t25381 + 6.0_f64 * t92295 * t25015 + t92299 + t1877 * t24344 * t87961 + 3.0_f64 / 2.0_f64 * t2522 * t24335 * t7475 - t1877 * t24339 * t25377 - 3.0_f64 * t24191 * t86810;
    (t92271, t92276, t92295, t92299, t92309)
}
