//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 561/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk561<F: Float>(t2586: F, t2936: F, t2508: F, t3448: F, t7129: F, t3420: F, t1024: F, t2717: F, t2927: F, t954: F, t7137: F, t3459: F, t841: F, t1052: F, t2728: F, t1022: F, t830: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10763 = t2936 * t2586;
    let t10765 = 0.23071578690426672851e-1 * t2508 * t10763;
    let t10767 = 0.15381052460284448567e-1 * t7129 * t3448;
    let t10769 = 0.76905262301422242837e-2 * t7129 * t3420;
    let t10770 = t2717 * t1024;
    let t10772 = 0.76905262301422242837e-2 * t2508 * t10770;
    let t10773 = t954 * t2927;
    let t10775 = 0.76905262301422242837e-2 * t2508 * t10773;
    let t10788 = 0.20508069947045931423e-1 * t7137 * t3448;
    let t10802 = t3459 * t841;
    let t10805 = t1052 * t2728;
    let t10809 = t830 * t1022;
    (t10765, t10767, t10769, t10772, t10775, t10788, t10802, t10805, t10809)
}
