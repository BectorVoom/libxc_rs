//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1228/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1228(t8104: f64, t8397: f64, t2138: f64, t2147: f64, t463: f64, t9435: f64, t7987: f64, t9432: f64, t157: f64, t1658: f64, t2143: f64, t2146: f64, t2152: f64, t2217: f64, t2385: f64, t2400: f64, t2404: f64, t29994: f64, t33063: f64, t33065: f64, t33175: f64, t33727: f64, t33739: f64, t524: f64, t7931: f64, t7938: f64, t8301: f64, t8306: f64, t8440: f64, t929: f64, t9422: f64, t9428: f64) -> f64 {
    let t38111 = t8397 * t8104;
    let t38138 = 0.34694512752820797848e1_f64 * t2138 * t2147 * t9435 * t463;
    let t38140 = 0.17347256376410398924e1_f64 * t7987 * t9432;
    let t38149 = -0.8673628188205199462e0_f64 * t38111 + 0.4336814094102599731e0_f64 * t2146 * t2152 * t8301 * t524 * t157 - 0.8673628188205199462e0_f64 * t2143 * t9422 + 0.13170898365871023197e1_f64 * t33063 + 0.26341796731742046394e1_f64 * t33065 + 0.17347256376410398924e1_f64 * t2146 * t2147 * t2217 * t1658 - 0.17347256376410398924e1_f64 * t33727 * t9428 - 0.4336814094102599731e0_f64 * t7938 * t2404 - 0.17347256376410398924e1_f64 * t7931 * t33175 * t8440 + 0.4336814094102599731e0_f64 * t29994 * t2400 - t38138 - t38140 + 0.4336814094102599731e0_f64 * t2146 * t2152 * t2385 * t929 * t157 - 0.8673628188205199462e0_f64 * t7931 * t8306 * t33739;
    t38149
}
