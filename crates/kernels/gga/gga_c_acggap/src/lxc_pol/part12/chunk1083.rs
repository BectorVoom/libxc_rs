//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1083/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1083<F: Float>(t33743: F, t33744: F, t638: F, t30029: F, t9171: F, t33175: F, t7942: F, t8406: F, t2176: F, t5368: F, t1620: F, t8331: F, t1222: F, t19834: F, t20138: F, t2222: F, t33163: F, t33240: F, t33250: F, t33735: F, t38086: F, t4109: F, t557: F, t7931: F, t7934: F, t8092: F, t8306: F, t8400: F, t9003: F, t9391: F, t9427: F) -> (F,) {
    let t38343 = 0.10408353825846239354e2 * t33743 * t638 * t33744;
    let t38345 = 0.17347256376410398924e1 * t30029 * t9171;
    let t38348 = 0.17347256376410398924e1 * t7942 * t33175 * t8406;
    let t38361 = t2176 * t5368;
    let t38370 = 0.26341796731742046394e1 * t8331 * t1620;
    let t38371 = 0.34694512752820797848e1 * t33240 + 0.8673628188205199462e0 * t8400 * t8306 * t19834 - t38343 - t38345 - t38348 + 0.4336814094102599731e0 * t9003 * t8092 - 0.65854491829355115987e0 * t33163 * t557 - 0.17347256376410398924e1 * t8400 * t9427 * t20138 - 0.8673628188205199462e0 * t8400 * t9427 * t33735 + 0.13170898365871023197e1 * t9391 * t1222 + 0.65854491829355115987e0 * t38361 - 0.17347256376410398924e1 * t7931 * t38086 * t7934 - 0.26341796731742046394e1 * t33250 - 0.39512695097613069591e1 * t2222 * t4109 + t38370;
    (t38371,)
}
