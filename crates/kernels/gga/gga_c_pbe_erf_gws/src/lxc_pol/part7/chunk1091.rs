//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1091/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1091<F: Float>(t43: F, t2092: F, t4347: F, t311: F, t19: F, t2331: F, t301: F, t305: F, t799: F, t19059: F, t19062: F, t19064: F, t19066: F, t19068: F, zeta_threshold: F) -> (F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t19528 = t2092 * t4347;
    let t19529 = F::new(0.73024584604562962965e1) * t19528;
    let t19530 = t311 * t311;
    let t19537 = F::new(0.34072858057724757727e0) * t305 / t19530 * t2331 * t301 * t19 * t799;
    let t19544 = piecewise3::<f64>(t44, F::new(0.0), -F::new(56.0) / F::new(81.0) * t19059 + F::new(16.0) / F::new(9.0) * t19062 - F::new(2.0) / F::new(3.0) * t19064 - F::new(8.0) / F::new(9.0) * t19066 + F::new(2.0) / F::new(3.0) * t19068);
    (t19529, t19537, t19544)
}
