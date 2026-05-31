//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1017/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1017<F: Float>(t2170: F, t2190: F, t3178: F, t2168: F, t6606: F, t6597: F, t9110: F, t9113: F, t9114: F, t9118: F, t9121: F, t9123: F, t9124: F, t9129: F, t9133: F) -> (F, F, F, F) {
    let t9135 = t2170 * t3178 * t2190;
    let t9137 = t2168 * t9135 / F::cast_from(48.0_f64);
    let t9138 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t6606;
    let t9139 = -t9110 - t9113 - t9114 - t9118 + t9121 + t9123 - t6597 - t9124 + t9129 + t9133 + t9137 + t9138;
    (t9135, t9137, t9138, t9139)
}
