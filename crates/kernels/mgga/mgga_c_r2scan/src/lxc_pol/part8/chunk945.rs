//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 945/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk945<F: Float>(t1862: F, t8987: F, t2743: F, t7654: F, t3165: F, t595: F, t637: F, t3162: F, t5754: F, t5761: F, t5766: F, t5770: F, t5901: F, t5907: F, t5910: F, t5912: F) -> (F, F, F, F, F, F, F) {
    let t8988 = t8987 * t1862;
    let t8990 = t2743 * t7654;
    let t8994 = t595 * t3165;
    let t8995 = t8994 * t637;
    let t8997 = t595 * t3162;
    let t8998 = t8997 * t637;
    let t9000 = -t5901 + 0.1350520664e0 * t8988 + 0.2701041328e0 * t8990 - t5754 + t5907 + 0.65061487801810439052e-1 * t5910 + 0.19263893255070628431e1 * t5912 - 0.40020429009866666666e-2 * t8995 - 0.20010214504933333333e-2 * t8998 + t5761 + t5766 + t5770;
    (t8988, t8990, t8994, t8995, t8997, t8998, t9000)
}
