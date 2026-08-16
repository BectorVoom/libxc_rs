//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 967/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk967<F: Float>(t10755: F, t1897: F, t3440: F, t7129: F, t8637: F, t948: F, t2508: F, t2586: F, t2936: F, t3448: F, t3420: F, t1024: F, t2717: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10757 = F::cast_from(0.76905262301422242837e-2_f64) * t1897 * t10755;
    let t10759 = F::cast_from(0.23071578690426672851e-1_f64) * t7129 * t3440;
    let t10760 = t8637 * t948;
    let t10762 = F::cast_from(0.23071578690426672851e-1_f64) * t2508 * t10760;
    let t10763 = t2936 * t2586;
    let t10765 = F::cast_from(0.23071578690426672851e-1_f64) * t2508 * t10763;
    let t10767 = F::cast_from(0.15381052460284448567e-1_f64) * t7129 * t3448;
    let t10769 = F::cast_from(0.76905262301422242837e-2_f64) * t7129 * t3420;
    let t10770 = t2717 * t1024;
    (t10757, t10759, t10760, t10762, t10763, t10765, t10767, t10769, t10770)
}
