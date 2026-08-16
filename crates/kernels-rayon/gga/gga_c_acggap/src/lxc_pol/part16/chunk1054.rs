//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1054/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1054(t157: f64, t1914: f64, t2122: f64, t2146: f64, t2152: f64, t26757: f64, t29973: f64, t29977: f64, t29982: f64, t32124: f64, t33414: f64, t33416: f64, t33431: f64, t33435: f64, t33437: f64, t38621: f64, t38631: f64, t38635: f64, t38639: f64, t7932: f64, t8004: f64, t8400: f64, t9033: f64) -> f64 {
    let t38641 = -0.26020884564615598386e1_f64 * t2146 * t8004 * t2122 * t1914 + t33414 - t33416 - t29973 + 0.4336814094102599731e0_f64 * t2146 * t2152 * t38621 * t157 - 0.8673628188205199462e0_f64 * t8400 * t9033 * t26757 - 0.26020884564615598386e1_f64 * t29977 + t33431 + t33435 - t33437 - 0.17347256376410398924e1_f64 * t38631 - 0.69389025505641595696e1_f64 * t29982 + 0.26020884564615598386e1_f64 * t32124 * t7932 * t38635 + 0.17347256376410398924e1_f64 * t38639;
    t38641
}
