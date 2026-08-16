//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 640/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk640<F: Float>(t10789: F, t740: F, t10751: F, t10754: F, t10757: F, t10759: F, t10762: F, t10765: F, t10767: F, t10769: F, t10772: F, t10775: F, t10776: F, t10779: F, t10784: F, t10788: F, t1897: F, t2508: F) -> F {
    let t10790 = t10789 * t740;
    let t10793 = t10751 + t10754 - t10757 - t10759 - t10762 - t10765 + t10767 + t10769 + t10772 + t10775 - F::cast_from(0.76905262301422242837e-2_f64) * t1897 * t10776 + F::cast_from(0.76905262301422242837e-2_f64) * t2508 * t10779 + F::cast_from(0.15381052460284448567e-1_f64) * t2508 * t10784 + t10788 - F::cast_from(0.23071578690426672851e-1_f64) * t2508 * t10790;
    t10793
}
