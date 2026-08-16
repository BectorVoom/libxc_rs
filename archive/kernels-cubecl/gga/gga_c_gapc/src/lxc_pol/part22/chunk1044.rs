//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1044/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1044<F: Float>(t12056: F, t1112: F, t3179: F, t1616: F, t1611: F, t3822: F, t687: F, t11314: F, t11318: F, t11323: F, t11327: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12057 = F::cast_from(2.0_f64) * t12056;
    let t12058 = t1112 * t3179;
    let t12059 = t1616 * t12058;
    let t12060 = F::cast_from(2.0_f64) * t12059;
    let t12061 = t1611 * t3822;
    let t12062 = t3822 * t687;
    let t12063 = t1616 * t12062;
    let t12064 = F::cast_from(2.0_f64) * t12063;
    let t12068 = F::cast_from(0.34752370105806885418e-3_f64) * t11314;
    let t12069 = F::cast_from(0.34752370105806885418e-3_f64) * t11318;
    let t12070 = F::cast_from(0.51491428373437201895e-5_f64) * t11323;
    let t12071 = F::cast_from(0.70344136651018351213e-8_f64) * t11327;
    (t12057, t12058, t12059, t12060, t12061, t12062, t12063, t12064, t12068, t12069, t12070, t12071)
}
