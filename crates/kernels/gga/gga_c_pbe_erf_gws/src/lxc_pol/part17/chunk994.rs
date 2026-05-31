//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 994/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk994<F: Float>(t8903: F, t8906: F, t2158: F, t3131: F, t3139: F, t3138: F, t3037: F, t339: F, t2306: F, t3074: F, t860: F, t8866: F, t8871: F, t8876: F, t8878: F, t8883: F, t8889: F, t8894: F, t8899: F, t8901: F) -> (F, F, F, F, F, F) {
    let t8908 = t8903 * t8906 / F::cast_from(16.0_f64);
    let t8910 = t3139 * t3131 * t2158;
    let t8912 = t3138 * t8910 / F::cast_from(16.0_f64);
    let t8913 = t3037 * t339;
    let t8914 = t2306 * t8913;
    let t8915 = t3074 * t8914;
    let t8917 = t8915 * t860 / F::cast_from(48.0_f64);
    let t8918 = t8866 + t8871 + t8876 - t8878 - t8883 - t8889 - t8894 + t8899 - t8901 - t8908 + t8912 + t8917;
    (t8908, t8910, t8912, t8913, t8917, t8918)
}
