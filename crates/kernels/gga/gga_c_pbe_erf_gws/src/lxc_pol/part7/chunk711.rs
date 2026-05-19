//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 711/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk711<F: Float>(t5683: F, t5825: F, t102: F, t1533: F, t497: F, t1235: F, t125: F, t128: F, t2: F, t39: F, t1563: F, t481: F) -> (F, F, F, F, F, F) {
    let t5826 = t5825 * t5683;
    let t5831 = F::new(0.1753815e2) * t102 * t497 * t1533;
    let t5832 = t125 * t1235;
    let t5833 = t128 * t2;
    let t5836 = F::cast_from(0.32645333333333333334e0_f64) * t5832 * t5833 * t39;
    let t5837 = t1563 * t481;
    (t5826, t5831, t5832, t5833, t5836, t5837)
}
