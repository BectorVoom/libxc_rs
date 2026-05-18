//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 839/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk839<F: Float>(t418: F, t7056: F, t7063: F, t7062: F, t1660: F, t597: F, t1663: F, t2647: F, t723: F, t2650: F, t4985: F, t4993: F) -> (F, F, F, F, F, F) {
    let t7064 = t7056 * t418;
    let t7065 = t7063 * t7064;
    let t7067 = F::new(16.0) / F::new(45.0) * t7062 * t7065;
    let t7068 = t1660 * t597;
    let t7069 = t7068 * t1663;
    let t7070 = t7069 * t7064;
    let t7072 = F::new(8.0) / F::new(27.0) * t7062 * t7070;
    let t7074 = F::new(4.0) / F::new(9.0) * t2647 * t723;
    let t7075 = t2650 * t723;
    let t7077 = F::new(8.0) / F::new(45.0) * t4985;
    let t7079 = F::new(16.0) / F::new(405.0) * t4993;
    (t7067, t7072, t7074, t7075, t7077, t7079)
}
