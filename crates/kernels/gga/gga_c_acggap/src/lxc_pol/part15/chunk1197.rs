//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1197/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1197<F: Float>(t119: F, t9971: F, t10011: F, t1914: F, t2146: F, t2217: F, t2404: F, t33063: F, t33065: F, t38092: F, t38138: F, t38140: F, t38647: F, t38784: F, t38827: F, t41089: F, t464: F, t7890: F, t7912: F, t7931: F, t8004: F, t8306: F, t8400: F, t9003: F, t9025: F, t9058: F, t9150: F, t944: F) -> F {
    let t41142 = t119 * t9971;
    let t41145 = -F::new(0.52041769129231196772e1) * t9003 * t9150 + F::new(0.65854491829355115987e0) * t33063 + F::new(0.13170898365871023197e1) * t33065 - F::new(0.8673628188205199462e0) * t9058 * t2404 + F::new(0.4336814094102599731e0) * t7912 * t10011 + F::new(0.8673628188205199462e0) * t8400 * t8306 * t38827 + F::new(0.4336814094102599731e0) * t8400 * t8306 * t38647 - F::new(0.17347256376410398924e1) * t7931 * t38092 * t9025 + F::new(0.4336814094102599731e0) * t8400 * t8306 * t38784 - F::new(0.8673628188205199462e0) * t2146 * t7890 * t41089 * t944 - t38138 - F::new(0.26020884564615598386e1) * t2146 * t8004 * t2217 * t1914 - t38140 - F::new(0.65854491829355115987e0) * t41142 * t464;
    t41145
}
