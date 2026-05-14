//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1076/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1076<F: Float>(t8104: F, t8397: F, t2138: F, t2147: F, t463: F, t9435: F, t7987: F, t9432: F, t157: F, t1658: F, t2143: F, t2146: F, t2152: F, t2217: F, t2385: F, t2400: F, t2404: F, t29994: F, t33063: F, t33065: F, t33175: F, t33727: F, t33739: F, t524: F, t7931: F, t7938: F, t8301: F, t8306: F, t8440: F, t929: F, t9422: F, t9428: F) -> (F,) {
    let t38111 = t8397 * t8104;
    let t38138 = 0.34694512752820797848e1 * t2138 * t2147 * t9435 * t463;
    let t38140 = 0.17347256376410398924e1 * t7987 * t9432;
    let t38149 = -0.8673628188205199462e0 * t38111 + 0.4336814094102599731e0 * t2146 * t2152 * t8301 * t524 * t157 - 0.8673628188205199462e0 * t2143 * t9422 + 0.13170898365871023197e1 * t33063 + 0.26341796731742046394e1 * t33065 + 0.17347256376410398924e1 * t2146 * t2147 * t2217 * t1658 - 0.17347256376410398924e1 * t33727 * t9428 - 0.4336814094102599731e0 * t7938 * t2404 - 0.17347256376410398924e1 * t7931 * t33175 * t8440 + 0.4336814094102599731e0 * t29994 * t2400 - t38138 - t38140 + 0.4336814094102599731e0 * t2146 * t2152 * t2385 * t929 * t157 - 0.8673628188205199462e0 * t7931 * t8306 * t33739;
    (t38149,)
}
