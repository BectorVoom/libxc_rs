//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 934/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk934<F: Float>(t3455: F, t582: F, t185: F, t2705: F, t954: F, t7194: F, t1620: F, t2570: F, t34: F, t2612: F, t2685: F, t2572: F, t7527: F) -> (F, F, F, F, F) {
    let t10485 = t582 * t3455;
    let t10486 = t185 * t10485;
    let t10487 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t10486;
    let t10488 = t2705 * t954;
    let t10489 = t7194 * t10488;
    let t10491 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t1620 * t10489;
    let t10492 = t2570 * t34;
    let t10493 = t7194 * t10492;
    let t10495 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t1620 * t10493;
    let t10497 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t2612 * t2685;
    let t10499 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t7527 * t2572;
    (t10487, t10491, t10495, t10497, t10499)
}
