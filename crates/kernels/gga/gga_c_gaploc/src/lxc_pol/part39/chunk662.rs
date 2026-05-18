//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 662/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk662<F: Float>(t10760: F, t2508: F, t2586: F, t2936: F, t3448: F, t7129: F, t3420: F, t1024: F, t2717: F, t2927: F, t954: F, t3464: F, t702: F) -> (F, F, F, F, F, F, F) {
    let t10762 = F::new(0.23071578690426672851e-1) * t2508 * t10760;
    let t10763 = t2936 * t2586;
    let t10765 = F::new(0.23071578690426672851e-1) * t2508 * t10763;
    let t10767 = F::new(0.15381052460284448567e-1) * t7129 * t3448;
    let t10769 = F::new(0.76905262301422242837e-2) * t7129 * t3420;
    let t10770 = t2717 * t1024;
    let t10772 = F::new(0.76905262301422242837e-2) * t2508 * t10770;
    let t10773 = t954 * t2927;
    let t10775 = F::new(0.76905262301422242837e-2) * t2508 * t10773;
    let t10776 = t3464 * t702;
    (t10762, t10765, t10767, t10769, t10772, t10775, t10776)
}
