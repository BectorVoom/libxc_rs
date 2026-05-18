//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 916/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk916<F: Float>(t4753: F, t4755: F, t1326: F, t959: F, t40: F, t6964: F, t85: F, t4785: F, t4791: F, t4794: F, t4796: F, t1444: F, t2506: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7994 = F::new(12.0) * t4753;
    let t7995 = F::new(24.0) * t4755;
    let t7996 = t959 * t1326;
    let t7997 = t40 * t7996;
    let t7998 = t6964 * t85;
    let t7999 = F::new(0.19751789702565206229e-1) * t7998;
    let t8000 = F::new(0.11696446794910408142e1) * t4785;
    let t8001 = F::new(0.34631511798751726598e2) * t4791;
    let t8002 = F::new(0.58482233974552040708e0) * t4794;
    let t8003 = F::new(0.23392893589820816284e1) * t4796;
    let t8004 = t2506 * t1444;
    (t7994, t7995, t7997, t7999, t8000, t8001, t8002, t8003, t8004)
}
