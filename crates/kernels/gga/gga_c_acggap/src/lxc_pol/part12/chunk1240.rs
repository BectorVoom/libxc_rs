//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1240/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1240<F: Float>(t1222: F, t19834: F, t20138: F, t2222: F, t33163: F, t33240: F, t33250: F, t33735: F, t38086: F, t38343: F, t38345: F, t38348: F, t38361: F, t38370: F, t4109: F, t557: F, t7931: F, t7934: F, t8092: F, t8306: F, t8400: F, t9003: F, t9391: F, t9427: F) -> F {
    let t38371 = F::new(0.34694512752820797848e1) * t33240 + F::new(0.8673628188205199462e0) * t8400 * t8306 * t19834 - t38343 - t38345 - t38348 + F::new(0.4336814094102599731e0) * t9003 * t8092 - F::new(0.65854491829355115987e0) * t33163 * t557 - F::new(0.17347256376410398924e1) * t8400 * t9427 * t20138 - F::new(0.8673628188205199462e0) * t8400 * t9427 * t33735 + F::new(0.13170898365871023197e1) * t9391 * t1222 + F::new(0.65854491829355115987e0) * t38361 - F::new(0.17347256376410398924e1) * t7931 * t38086 * t7934 - F::new(0.26341796731742046394e1) * t33250 - F::new(0.39512695097613069591e1) * t2222 * t4109 + t38370;
    t38371
}
