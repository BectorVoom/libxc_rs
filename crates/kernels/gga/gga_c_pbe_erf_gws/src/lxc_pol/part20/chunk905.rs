//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 905/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk905<F: Float>(t1563: F, t3637: F, t102: F, t3656: F, t481: F, t2873: F, t974: F, t3660: F, t10102: F, t10106: F, t10107: F, t10110: F, t127: F, t2893: F, t5836: F, t8200: F) -> (F, F, F, F) {
    let t10117 = t1563 * t3637;
    let t10123 = F::cast_from(0.1753815e2_f64) * t102 * t3656 * t481;
    let t10126 = F::cast_from(0.116921e2_f64) * t102 * t974 * t2873;
    let t10129 = F::cast_from(0.584605e1_f64) * t102 * t3660 * t481;
    let t10130 = -t10102 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t8200 + t5836 - t10106 - F::cast_from(0.146904e1_f64) * t127 * t10107 - F::cast_from(0.293808e2_f64) * t127 * t10110 * t481 + F::cast_from(0.1175232e2_f64) * t127 * t2893 * t2873 + F::cast_from(0.587616e1_f64) * t127 * t10117 * t481 - t10123 + t10126 + t10129;
    (t10123, t10126, t10129, t10130)
}
