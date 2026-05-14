//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 900/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk900<F: Float>(t3074: F, t8914: F, t860: F, t8866: F, t8871: F, t8876: F, t8878: F, t8883: F, t8889: F, t8894: F, t8899: F, t8901: F, t8908: F, t8912: F, t3131: F, t3139: F, t6360: F) -> (F, F, F) {
    let t8915 = t3074 * t8914;
    let t8917 = t8915 * t860 / 48.0;
    let t8918 = t8866 + t8871 + t8876 - t8878 - t8883 - t8889 - t8894 + t8899 - t8901 - t8908 + t8912 + t8917;
    let t8921 = t3139 * t3131 * t6360;
    (t8917, t8918, t8921)
}
