//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 756/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk756<F: Float>(t1023: F, t35385: F, t1381: F, t3549: F, t11699: F, t747: F, t3516: F, t475: F, t3529: F, t2366: F, t6508: F, t172: F, t2754: F) -> (F, F, F, F, F, F, F) {
    let t35719 = t1023 * t35385;
    let t35770 = t3549 * t1381;
    let t35781 = t11699 * t747;
    let t35845 = t3516 * t475;
    let t35887 = t3529 * t475;
    let t35888 = t2366 * t35887;
    let t35893 = t6508 * t35887;
    let t35900 = t172 * t2754;
    (t35719, t35770, t35781, t35845, t35888, t35893, t35900)
}
