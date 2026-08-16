//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1198/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1198(t38092: f64, t7963: f64, t9029: f64, t7942: f64, t8406: f64, t10025: f64, t157: f64, t2146: f64, t2152: f64, t31965: f64, t33080: f64, t33093: f64, t33097: f64, t33100: f64, t33104: f64, t38153: f64, t38157: f64, t38165: f64, t38176: f64, t38685: f64, t40675: f64, t6068: f64, t633: f64, t7931: f64, t8306: f64) -> f64 {
    let t41164 = t7963 * t38092 * t9029;
    let t41167 = t7942 * t38092 * t8406;
    let t41169 = t38153 + t38157 - 0.13170898365871023197e1_f64 * t33080 + t38165 + 0.4336814094102599731e0_f64 * t2146 * t2152 * t633 * t6068 * t157 - 0.65854491829355115987e0_f64 * t33093 - 0.8673628188205199462e0_f64 * t33097 - t33100 - 0.8673628188205199462e0_f64 * t7931 * t8306 * t38685 - 0.17347256376410398924e1_f64 * t7931 * t8306 * t40675 + t38176 - 0.17347256376410398924e1_f64 * t31965 * t10025 + 0.8673628188205199462e0_f64 * t33104 + 0.17347256376410398924e1_f64 * t41164 - 0.17347256376410398924e1_f64 * t41167;
    t41169
}
