//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 817/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk817(t157: f64, t2385: f64, t406: f64, t2152: f64, t1620: f64, t2143: f64, t2146: f64, t2222: f64, t2245: f64, t2338: f64, t2404: f64, t8062: f64, t8067: f64, t8074: f64, t8076: f64, t8078: f64, t8082: f64, t8087: f64, t8096: f64, t9129: f64, t9136: f64) -> (f64, f64) {
    let t9144 = t2385 * t406 * t157;
    let t9145 = t2152 * t9144;
    let t9148 = 0.8673628188205199462e0_f64 * t8062 + t8067 + 0.17347256376410398924e1_f64 * t8074 + 0.13170898365871023197e1_f64 * t2222 * t1620 + 0.65854491829355115987e0_f64 * t9129 + 0.8673628188205199462e0_f64 * t8076 - 0.8673628188205199462e0_f64 * t8078 - 0.17347256376410398924e1_f64 * t8082 + t8087 + 0.4336814094102599731e0_f64 * t2146 * t9136 - t8096 - 0.4336814094102599731e0_f64 * t2338 * t2245 - 0.4336814094102599731e0_f64 * t2143 * t2404 + 0.4336814094102599731e0_f64 * t2146 * t9145;
    (t9145, t9148)
}
