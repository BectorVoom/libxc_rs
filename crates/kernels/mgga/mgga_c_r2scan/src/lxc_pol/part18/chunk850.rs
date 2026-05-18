//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 850/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk850<F: Float>(t1859: F, t3033: F, t1862: F, t2743: F, t7654: F, t3165: F, t595: F, t637: F, t3162: F, t5754: F, t5761: F, t5766: F, t5770: F, t5901: F, t5907: F, t5910: F, t5912: F) -> F {
    let t8987 = t1859 * t3033;
    let t8988 = t8987 * t1862;
    let t8990 = t2743 * t7654;
    let t8994 = t595 * t3165;
    let t8995 = t8994 * t637;
    let t8997 = t595 * t3162;
    let t8998 = t8997 * t637;
    let t9000 = -t5901 + F::new(0.1350520664e0) * t8988 + F::new(0.2701041328e0) * t8990 - t5754 + t5907 + F::new(0.65061487801810439052e-1) * t5910 + F::new(0.19263893255070628431e1) * t5912 - F::new(0.40020429009866666666e-2) * t8995 - F::new(0.20010214504933333333e-2) * t8998 + t5761 + t5766 + t5770;
    t9000
}
