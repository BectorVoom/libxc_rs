//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 503/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk503<F: Float>(t123: F, t7284: F, t2563: F, t9647: F, t5539: F, t7292: F, t1841: F, t3244: F, t3252: F, t3256: F, t681: F, t9627: F, t9629: F, t9632: F, t9635: F, t9638: F, t9643: F) -> (F, F, F, F) {
    let t9648 = t7284 * t123;
    let t9649 = t9648 * t2563;
    let t9651 = 0.1922631557535556071e-2 * t9647 * t9649;
    let t9652 = t5539 * t7292;
    let t9654 = 0.1281754371690370714e-2 * t9647 * t9652;
    let t9661 = -t9627 + t9629 + t9632 - t9635 - 0.85450291446024714263e-3 * t1841 * t9638 - 0.85450291446024714263e-3 * t1841 * t9643 - t9651 + t9654 - 0.23071578690426672851e-1 * t681 * t3244 + 0.15381052460284448567e-1 * t681 * t3252 - 0.76905262301422242837e-2 * t681 * t3256;
    (t9648, t9651, t9654, t9661)
}
