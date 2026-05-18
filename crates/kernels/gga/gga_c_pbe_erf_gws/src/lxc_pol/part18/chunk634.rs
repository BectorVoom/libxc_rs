//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 634/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk634<F: Float>(t995: F, t181: F, t184: F, t199: F, t2570: F, t954: F, t1809: F, t1620: F, t1027: F, t1044: F, t1815: F, t639: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3397 = t995 * t995;
    let t3398 = t3397 * t181;
    let t3399 = t3398 * t184;
    let t3401 = F::new(4.0) / F::new(15.0) * t3399 * t199;
    let t3402 = t2570 * t954;
    let t3403 = t1809 * t3402;
    let t3405 = F::new(16.0) / F::new(45.0) * t1620 * t3403;
    let t3406 = t1027 * t1044;
    let t3407 = t1815 * t3406;
    let t3409 = F::new(8.0) / F::new(45.0) * t639 * t3407;
    (t3397, t3398, t3399, t3401, t3402, t3403, t3405, t3406, t3407, t3409)
}
