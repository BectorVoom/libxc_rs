//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 973/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk973<F: Float>(t10979: F, t587: F, t10792: F, t1821: F, t10796: F, t7694: F, t2768: F, t950: F, t1820: F, t3414: F, t5129: F, t2688: F, t7495: F) -> (F, F, F, F, F, F) {
    let t10981 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t587 * t10979;
    let t10982 = t1821 * t10792;
    let t10984 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t587 * t10982;
    let t10985 = t7694 * t10796;
    let t10987 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t587 * t10985;
    let t10988 = t2768 * t950;
    let t10989 = t7694 * t10988;
    let t10991 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t1820 * t10989;
    let t10992 = t5129 * t3414;
    let t10993 = t587 * t10992;
    let t10994 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t10993;
    let t10995 = t7495 * t2688;
    (t10981, t10984, t10987, t10991, t10994, t10995)
}
