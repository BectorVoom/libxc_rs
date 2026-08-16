//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1199/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1199(t33799: f64, t9171: f64, t32130: f64, t38052: f64, t9029: f64, t40733: f64, t7963: f64, t8306: f64, t2143: f64, t2146: f64, t2147: f64, t2222: f64, t33107: f64, t33118: f64, t33120: f64, t38181: f64, t38185: f64, t38187: f64, t38190: f64, t38194: f64, t557: f64, t633: f64, t6557: f64, t6569: f64, t9995: f64) -> f64 {
    let t41176 = t33799 * t9171;
    let t41187 = t32130 * t38052 * t9029;
    let t41192 = t7963 * t8306 * t40733;
    let t41194 = -t33107 - 0.13877805101128319139e2_f64 * t38181 + 0.13170898365871023197e1_f64 * t2222 * t6569 - 0.4336814094102599731e0_f64 * t2143 * t9995 - 0.17347256376410398924e1_f64 * t41176 + 0.69389025505641595696e1_f64 * t38185 + 0.34694512752820797848e1_f64 * t38190 - t38194 - 0.26020884564615598386e1_f64 * t33118 + 0.8673628188205199462e0_f64 * t2146 * t2147 * t633 * t6557 - 0.26020884564615598386e1_f64 * t33120 - 0.34694512752820797848e1_f64 * t41187 - 0.13170898365871023197e1_f64 * t38187 * t557 + 0.8673628188205199462e0_f64 * t41192;
    t41194
}
