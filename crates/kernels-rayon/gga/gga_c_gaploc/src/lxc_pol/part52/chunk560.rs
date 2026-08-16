//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 560/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk560(t8637: f64, t948: f64, t2508: f64, t2586: f64, t2936: f64, t3448: f64, t7129: f64, t3420: f64, t1024: f64, t2717: f64, t2927: f64, t954: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10760 = t8637 * t948;
    let t10762 = 0.23071578690426672851e-1_f64 * t2508 * t10760;
    let t10763 = t2936 * t2586;
    let t10765 = 0.23071578690426672851e-1_f64 * t2508 * t10763;
    let t10767 = 0.15381052460284448567e-1_f64 * t7129 * t3448;
    let t10769 = 0.76905262301422242837e-2_f64 * t7129 * t3420;
    let t10770 = t2717 * t1024;
    let t10772 = 0.76905262301422242837e-2_f64 * t2508 * t10770;
    let t10773 = t954 * t2927;
    (t10762, t10765, t10767, t10769, t10772, t10773)
}
