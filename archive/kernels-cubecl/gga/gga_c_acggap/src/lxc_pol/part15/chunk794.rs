//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 794/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk794<F: Float>(t157: F, t2385: F, t406: F, t2152: F, t1620: F, t2143: F, t2146: F, t2222: F, t2245: F, t2338: F, t2404: F, t8062: F, t8067: F, t8074: F, t8076: F, t8078: F, t8082: F, t8087: F, t8096: F, t9129: F, t9136: F) -> (F, F) {
    let t9144 = t2385 * t406 * t157;
    let t9145 = t2152 * t9144;
    let t9148 = F::cast_from(0.8673628188205199462e0_f64) * t8062 + t8067 + F::cast_from(0.17347256376410398924e1_f64) * t8074 + F::cast_from(0.13170898365871023197e1_f64) * t2222 * t1620 + F::cast_from(0.65854491829355115987e0_f64) * t9129 + F::cast_from(0.8673628188205199462e0_f64) * t8076 - F::cast_from(0.8673628188205199462e0_f64) * t8078 - F::cast_from(0.17347256376410398924e1_f64) * t8082 + t8087 + F::cast_from(0.4336814094102599731e0_f64) * t2146 * t9136 - t8096 - F::cast_from(0.4336814094102599731e0_f64) * t2338 * t2245 - F::cast_from(0.4336814094102599731e0_f64) * t2143 * t2404 + F::cast_from(0.4336814094102599731e0_f64) * t2146 * t9145;
    (t9145, t9148)
}
