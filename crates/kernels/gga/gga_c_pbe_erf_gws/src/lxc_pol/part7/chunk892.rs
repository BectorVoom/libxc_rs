//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 892/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk892<F: Float>(t16712: F, t197: F, t16669: F, t5293: F, t587: F, t1820: F, t5018: F, t5300: F, t16907: F, t16910: F, t16912: F, t16917: F, t16921: F, t16925: F, t16927: F, t16929: F, t16931: F) -> (F, F, F) {
    let t16932 = t197 * t16712;
    let t16936 = F::cast_from(128.0_f64) / F::cast_from(27.0_f64) * t587 * t5293 * t16932 * t16669;
    let t16938 = t1820 * t5018 * t5300;
    let t16939 = F::cast_from(64.0_f64) / F::cast_from(15.0_f64) * t16938;
    let t16940 = -t16907 - t16910 - t16912 - t16917 + t16921 + t16925 + t16927 - t16929 - t16931 - t16936 + t16939;
    (t16936, t16939, t16940)
}
