//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 880/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk880<F: Float>(t10972: F, t561: F, t5513: F, t1006: F, t2786: F, t3425: F, t610: F, t1827: F, t587: F, t10792: F, t1821: F, t10796: F, t7694: F, t2768: F, t950: F, t1820: F) -> (F, F, F, F, F, F, F) {
    let t10973 = t561 * t10972;
    let t10974 = 8.0 / 45.0 * t10973;
    let t10975 = 4.0 / 135.0 * t5513;
    let t10977 = 4.0 / 15.0 * t1006 * t2786;
    let t10978 = t3425 * t610;
    let t10979 = t1827 * t10978;
    let t10981 = 8.0 / 45.0 * t587 * t10979;
    let t10982 = t1821 * t10792;
    let t10984 = 8.0 / 15.0 * t587 * t10982;
    let t10985 = t7694 * t10796;
    let t10987 = 32.0 / 45.0 * t587 * t10985;
    let t10988 = t2768 * t950;
    let t10989 = t7694 * t10988;
    let t10991 = 16.0 / 45.0 * t1820 * t10989;
    (t10974, t10975, t10977, t10981, t10984, t10987, t10991)
}
