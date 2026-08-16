//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 640/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk640(t10789: f64, t740: f64, t10751: f64, t10754: f64, t10757: f64, t10759: f64, t10762: f64, t10765: f64, t10767: f64, t10769: f64, t10772: f64, t10775: f64, t10776: f64, t10779: f64, t10784: f64, t10788: f64, t1897: f64, t2508: f64) -> f64 {
    let t10790 = t10789 * t740;
    let t10793 = t10751 + t10754 - t10757 - t10759 - t10762 - t10765 + t10767 + t10769 + t10772 + t10775 - 0.76905262301422242837e-2_f64 * t1897 * t10776 + 0.76905262301422242837e-2_f64 * t2508 * t10779 + 0.15381052460284448567e-1_f64 * t2508 * t10784 + t10788 - 0.23071578690426672851e-1_f64 * t2508 * t10790;
    t10793
}
