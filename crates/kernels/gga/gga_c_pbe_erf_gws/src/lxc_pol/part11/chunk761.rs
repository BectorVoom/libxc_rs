//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 761/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk761<F: Float>(t1004: F, t3397: F, t184: F, t199: F, t12339: F, t4951: F, t4949: F, t11: F, t4957: F, t1758: F, t2560: F, t3346: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12451 = t3397 * t1004;
    let t12452 = t12451 * t184;
    let t12454 = F::new(4.0) / F::new(5.0) * t12452 * t199;
    let t12460 = t4951 * t12339;
    let t12461 = t4949 * t12460;
    let t12462 = t11 * t12461;
    let t12464 = t4957 * t12339;
    let t12465 = t1758 * t12464;
    let t12466 = t11 * t12465;
    let t12468 = t2560 * t3346;
    (t12451, t12452, t12454, t12460, t12461, t12462, t12464, t12465, t12466, t12468)
}
