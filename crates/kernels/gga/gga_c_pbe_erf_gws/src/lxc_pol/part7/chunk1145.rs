//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1145/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1145<F: Float>(t20509: F, t6680: F, t2387: F, t6671: F, t6674: F, t2158: F, t814: F, t2118: F, t2250: F, t6201: F, t933: F, t6279: F) -> (F, F, F, F, F) {
    let t20511 = t20509 * t6680 / F::cast_from(12.0_f64);
    let t20512 = t2387 * t6671;
    let t20514 = t20512 * t6674 / F::cast_from(4.0_f64);
    let t20515 = t2158 * t814;
    let t20516 = t2118 * t20515;
    let t20521 = t2250 * t6201 * t933;
    let t20522 = t20521 * t6279;
    (t20511, t20514, t20515, t20516, t20522)
}
