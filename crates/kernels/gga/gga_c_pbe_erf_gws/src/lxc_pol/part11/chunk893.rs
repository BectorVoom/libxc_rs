//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 893/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk893<F: Float>(t197: F, t4991: F, t1661: F, t1802: F, t5480: F, t649: F, t16984: F, t1697: F, t191: F, t205: F, t190: F, t212: F, t367: F) -> (F, F, F, F, F, F) {
    let t17819 = t4991 * t197;
    let t17852 = t1661 * t1802;
    let t17870 = t5480 * t649;
    let t17900 = F::new(0.37324691358024691357e0) * t16984;
    let t17957 = t191 / t205 / t1697;
    let t17983 = F::new(0.10864197530864197531e0) * t190 * t367 * t212;
    (t17819, t17852, t17870, t17900, t17957, t17983)
}
