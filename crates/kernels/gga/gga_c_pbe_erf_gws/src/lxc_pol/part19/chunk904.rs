//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 904/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk904<F: Float>(t1563: F, t3637: F, t102: F, t3656: F, t481: F, t2873: F, t974: F, t3660: F, t10102: F, t10106: F, t10107: F, t10110: F, t127: F, t2893: F, t5836: F, t8200: F) -> (F, F, F, F) {
    let t10117 = t1563 * t3637;
    let t10123 = F::new(0.1753815e2) * t102 * t3656 * t481;
    let t10126 = F::new(0.116921e2) * t102 * t974 * t2873;
    let t10129 = F::new(0.584605e1) * t102 * t3660 * t481;
    let t10130 = -t10102 - F::new(4.0) / F::new(9.0) * t8200 + t5836 - t10106 - F::new(0.146904e1) * t127 * t10107 - F::new(0.293808e2) * t127 * t10110 * t481 + F::new(0.1175232e2) * t127 * t2893 * t2873 + F::new(0.587616e1) * t127 * t10117 * t481 - t10123 + t10126 + t10129;
    (t10123, t10126, t10129, t10130)
}
