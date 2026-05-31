//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1136/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1136<F: Float>(t41385: F, t41388: F, t41395: F, t41398: F, t41401: F, t41404: F, t48092: F, t48095: F, t48099: F, t48101: F, t48102: F, t48103: F) -> (F, F, F, F, F, F, F) {
    let t48104 = F::cast_from(64.0_f64) / F::cast_from(45.0_f64) * t41385;
    let t48105 = F::cast_from(128.0_f64) / F::cast_from(45.0_f64) * t41388;
    let t48106 = F::cast_from(128.0_f64) / F::cast_from(45.0_f64) * t41395;
    let t48107 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t41398;
    let t48108 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t41401;
    let t48109 = F::cast_from(64.0_f64) / F::cast_from(27.0_f64) * t41404;
    let t48110 = t48092 - t48095 - t48099 - t48101 - t48102 + t48103 - t48104 - t48105 - t48106 + t48107 - t48108 - t48109;
    (t48104, t48105, t48106, t48107, t48108, t48109, t48110)
}
