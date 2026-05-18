//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1071/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1071<F: Float>(t12039: F, t326: F, t9385: F, t6252: F, t3037: F, t5: F, t337: F, t2121: F, t1076: F, t814: F, t2255: F, t3258: F) -> (F, F, F, F, F) {
    let t12040 = F::new(7.0) / F::new(288.0) * t12039;
    let t12041 = t326 * t9385;
    let t12042 = t12041 * t6252;
    let t12043 = t5 * t3037;
    let t12044 = t337 * t12043;
    let t12045 = t2121 * t12044;
    let t12047 = t12042 * t12045 / F::new(48.0);
    let t12048 = t1076 * t814;
    let t12050 = t2255 * t3258 * t12048;
    (t12040, t12041, t12044, t12047, t12050)
}
