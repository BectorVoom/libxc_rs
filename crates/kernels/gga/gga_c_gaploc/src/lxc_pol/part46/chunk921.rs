//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 921/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk921<F: Float>(t42898: F, t42863: F, t42866: F, t42867: F, t42868: F, t42869: F, t42870: F, t42871: F, t42872: F, t42873: F, t42874: F, t42877: F, t42881: F, t42883: F, t42885: F, t42889: F, t42893: F, t42896: F) -> F {
    let t42899 = F::cast_from(0.23712505529730124666e-2_f64) * t42898;
    let t42900 = t42863 + t42866 - t42867 + t42868 - t42869 - t42870 - t42871 + t42872 + t42873 + t42874 - t42877 + t42881 - F::cast_from(0.56910013271352299198e-1_f64) * t42883 - F::cast_from(0.23712505529730124666e-2_f64) * t42885 - F::cast_from(0.23712505529730124666e-2_f64) * t42889 - t42893 + t42896 + t42899;
    t42900
}
