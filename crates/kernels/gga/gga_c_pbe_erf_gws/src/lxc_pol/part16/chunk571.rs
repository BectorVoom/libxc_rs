//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 571/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk571<F: Float>(t43: F, t2474: F, t85: F, t1523: F, t950: F, t418: F, t34: F, t476: F, t532: F, t1528: F, t954: F, t422: F, t478: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t2475 = t2474 * t85;
    let t2476 = F::cast_from(0.19751789702565206229e-1_f64) * t2475;
    let t2477 = t1523 * t950;
    let t2478 = t2477 * t418;
    let t2480 = t476 * t34;
    let t2481 = t2480 * t532;
    let t2484 = piecewise3::<F>(t44, F::new(0.0), -F::new(2.0) / F::new(9.0) * t2478 + F::new(4.0) / F::new(3.0) * t2481);
    let t2485 = t1528 * t954;
    let t2486 = t2485 * t422;
    let t2488 = t478 * t34;
    (t2476, t2477, t2478, t2480, t2481, t2484, t2485, t2486, t2488)
}
