//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 600/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk600<F: Float>(t384: F, t4503: F, t1150: F, t3215: F, t3218: F, t3229: F, t3231: F, t3233: F, t3235: F, t3238: F, t3240: F, t3246: F, t3271: F, t3273: F, t3280: F, t3293: F, t335: F, t3616: F, t367: F, t4480: F, t4484: F, t4488: F, t4492: F, t4494: F, t4496: F) -> F {
    let t4505 = F::cast_from(0.85748036236139473944e-3_f64) * t384 * t4503;
    let t4507 = -t3215 - t3218 - F::cast_from(0.17149607247227894789e-2_f64) * t3229 + F::cast_from(0.85748036236139473944e-3_f64) * t3231 - F::cast_from(0.85748036236139473944e-3_f64) * t3233 + F::cast_from(0.40015750243531754508e-2_f64) * t3235 - F::cast_from(0.80031500487063509016e-2_f64) * t3238 + F::cast_from(0.80031500487063509016e-2_f64) * t3240 - t3246 + t367 * t4480 / F::new(48.0) + t1150 * t4484 / F::new(16.0) - t3616 * t4488 / F::new(4.0) - t4492 - t4494 + t335 * t4496 / F::new(48.0) + F::cast_from(0.42874018118069736972e-3_f64) * t3271 - F::cast_from(0.85748036236139473944e-3_f64) * t3273 - F::cast_from(0.20007875121765877254e-2_f64) * t3280 - t4505 - F::cast_from(0.12862205435420921092e-2_f64) * t3293;
    t4507
}
