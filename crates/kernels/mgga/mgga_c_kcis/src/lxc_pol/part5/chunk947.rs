//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 947/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk947<F: Float>(t2389: F, t2415: F, t705: F, t2387: F, t2388: F, t78: F, t686: F, t60: F, t2381: F, t2386: F, t2390: F, t2416: F, t688: F, t82: F, t8747: F, t8949: F, t8957: F, t8961: F, t8965: F, t9001: F) -> F {
    let t9236 = t2415 * t2389;
    let t9237 = t9236 * t705;
    let t9249 = t2387 * t705;
    let t9251 = F::new(1.0) / t2388 / t78;
    let t9252 = t9249 * t9251;
    let t9260 = t686 * t686;
    let t9261 = F::new(1.0) / t9260;
    let t9262 = t60 * t9261;
    let t9265 = F::new(0.200175e0) * t688 * t9237 + F::new(0.200175e0) * t2381 * t2390 + F::cast_from(0.59694999999999999999e-1_f64) * t8949 - F::new(0.200175e0) * t2381 * t2416 + t8747 * t82 + F::cast_from(0.92858888888888888885e-1_f64) * t8957 + F::cast_from(0.2671335375e-1_f64) * t2386 * t9237 - F::cast_from(0.2671335375e-1_f64) * t2386 * t9252 - F::cast_from(0.10317654320987654321e0_f64) * t8961 - F::cast_from(0.39796666666666666665e-1_f64) * t8965 - F::new(0.13345e0) * t688 * t9252 + F::cast_from(0.99491666666666666664e-2_f64) * t9001 - F::cast_from(0.178244852896875e-2_f64) * t9262 * t9252;
    t9265
}
