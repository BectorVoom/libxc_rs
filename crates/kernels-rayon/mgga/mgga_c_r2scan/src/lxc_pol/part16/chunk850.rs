//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 850/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk850(t1859: f64, t3033: f64, t1862: f64, t2743: f64, t7654: f64, t3165: f64, t595: f64, t637: f64, t3162: f64, t5754: f64, t5761: f64, t5766: f64, t5770: f64, t5901: f64, t5907: f64, t5910: f64, t5912: f64) -> f64 {
    let t8987 = t1859 * t3033;
    let t8988 = t8987 * t1862;
    let t8990 = t2743 * t7654;
    let t8994 = t595 * t3165;
    let t8995 = t8994 * t637;
    let t8997 = t595 * t3162;
    let t8998 = t8997 * t637;
    let t9000 = -t5901 + 0.1350520664e0_f64 * t8988 + 0.2701041328e0_f64 * t8990 - t5754 + t5907 + 0.65061487801810439052e-1_f64 * t5910 + 0.19263893255070628431e1_f64 * t5912 - 0.40020429009866666666e-2_f64 * t8995 - 0.20010214504933333333e-2_f64 * t8998 + t5761 + t5766 + t5770;
    t9000
}
