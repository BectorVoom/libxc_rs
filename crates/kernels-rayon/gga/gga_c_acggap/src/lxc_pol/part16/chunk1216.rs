//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1216/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1216(t2138: f64, t2147: f64, t322: f64, t9793: f64, t157: f64, t2127: f64, t2146: f64, t2152: f64, t29997: f64, t32082: f64, t32091: f64, t32109: f64, t32121: f64, t33794: f64, t33798: f64, t33801: f64, t33804: f64, t36405: f64, t36419: f64, t39794: f64, t6068: f64, t609: f64, t6569: f64, t7931: f64, t8400: f64, t9033: f64, t9508: f64) -> f64 {
    let t40824 = t2138 * t2147 * t9793 * t322;
    let t40837 = 0.13170898365871023197e1_f64 * t2127 * t6569 - t33794 + t33798 - t33801 - t33804 - 0.13170898365871023197e1_f64 * t32082 - 0.17347256376410398924e1_f64 * t7931 * t29997 * t9508 - t32091 - 0.17347256376410398924e1_f64 * t40824 - 0.26341796731742046394e1_f64 * t36405 - t32109 + 0.4336814094102599731e0_f64 * t2146 * t2152 * t609 * t6068 * t157 - 0.26020884564615598386e1_f64 * t8400 * t9033 * t39794 + 0.13170898365871023197e1_f64 * t32121 - 0.13877805101128319139e2_f64 * t36419;
    t40837
}
