//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1091/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1091<F: Float>(t13756: F, t380: F, t42873: F, t42874: F, t42877: F, t42881: F, t42883: F, t42885: F, t42889: F, t42893: F, t42896: F, t42899: F) -> F {
    let t47054 = F::new(0.37940008847568199465e-1) * t380 * t13756;
    let t47058 = t42873 + t42874 + t47054 - t42877 + t42881 - F::new(0.28455006635676149599e-1) * t42883 - F::new(0.11856252764865062333e-2) * t42885 - F::new(0.11856252764865062333e-2) * t42889 - t42893 + t42896 + t42899;
    t47058
}
