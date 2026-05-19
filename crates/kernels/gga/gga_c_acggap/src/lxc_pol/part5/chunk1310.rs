//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1310/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1310<F: Float>(t3088: F, t4183: F, t6461: F, t12313: F, t16559: F, t6465: F, t1004: F, t6455: F, t1251: F, t14539: F, t14547: F, t14551: F, t14554: F, t14556: F, t14564: F, t151: F, t19196: F, t19199: F, t19208: F, t19213: F, t6069: F) -> F {
    let t24397 = t3088 * t6461 * t4183;
    let t24400 = t12313 * t6465 * t16559;
    let t24410 = t1004 * t6455;
    let t24415 = -F::cast_from(0.26341796731742046394e1_f64) * t24397 + F::cast_from(0.26341796731742046394e1_f64) * t24400 + F::cast_from(0.39512695097613069592e1_f64) * t14539 + F::cast_from(0.39512695097613069592e1_f64) * t14547 - F::cast_from(0.13170898365871023197e1_f64) * t14551 + t14554 - F::cast_from(0.79025390195226139182e1_f64) * t19196 - F::cast_from(0.79025390195226139182e1_f64) * t19199 - F::cast_from(0.13170898365871023197e1_f64) * t151 * t1251 * t6069 - t14556 - F::cast_from(0.13170898365871023197e1_f64) * t24410 + F::cast_from(0.79025390195226139182e1_f64) * t19208 + F::cast_from(0.79025390195226139182e1_f64) * t14564 - F::cast_from(0.13170898365871023197e1_f64) * t19213;
    t24415
}
