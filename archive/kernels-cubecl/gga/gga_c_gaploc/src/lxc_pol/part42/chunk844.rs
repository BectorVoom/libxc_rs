//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 844/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk844<F: Float>(t13556: F, t7137: F, t11595: F, t1897: F, t7671: F, t13489: F, t2549: F, t11608: F, t2580: F, t7068: F, t2562: F, t35558: F, t883: F, t943: F) -> (F, F, F, F, F) {
    let t45000 = F::cast_from(0.20508069947045931423e-1_f64) * t7137 * t13556;
    let t45009 = F::cast_from(0.23071578690426672851e-1_f64) * t1897 * t11595 * t7671;
    let t45010 = t2549 * t13489;
    let t45015 = F::cast_from(0.15381052460284448567e-1_f64) * t1897 * t2580 * t11608 * t7068;
    let t45028 = t943 * t2562 * t883 * t35558;
    (t45000, t45009, t45010, t45015, t45028)
}
