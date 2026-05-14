//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 930/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk930<F: Float>(t42857: F, t42863: F, t42866: F, t42867: F, t42868: F, t42869: F, t42870: F, t42871: F, t42872: F, t47047: F, t47050: F, t13756: F, t380: F, t42873: F, t42874: F, t42877: F, t42881: F, t42883: F, t42885: F, t42889: F, t42893: F, t42896: F, t42899: F) -> (F, F) {
    let t47052 = -0.19918504644973304719e0 * t47047 + t42857 + t42863 + t42866 - t42867 + t42868 + 0.85365019907028448797e-1 * t47050 - t42869 - t42870 - t42871 + t42872;
    let t47054 = 0.37940008847568199465e-1 * t380 * t13756;
    let t47058 = t42873 + t42874 + t47054 - t42877 + t42881 - 0.28455006635676149599e-1 * t42883 - 0.11856252764865062333e-2 * t42885 - 0.11856252764865062333e-2 * t42889 - t42893 + t42896 + t42899;
    (t47052, t47058)
}
