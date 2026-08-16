//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1310/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1310(t3088: f64, t4183: f64, t6461: f64, t12313: f64, t16559: f64, t6465: f64, t1004: f64, t6455: f64, t1251: f64, t14539: f64, t14547: f64, t14551: f64, t14554: f64, t14556: f64, t14564: f64, t151: f64, t19196: f64, t19199: f64, t19208: f64, t19213: f64, t6069: f64) -> f64 {
    let t24397 = t3088 * t6461 * t4183;
    let t24400 = t12313 * t6465 * t16559;
    let t24410 = t1004 * t6455;
    let t24415 = -0.26341796731742046394e1_f64 * t24397 + 0.26341796731742046394e1_f64 * t24400 + 0.39512695097613069592e1_f64 * t14539 + 0.39512695097613069592e1_f64 * t14547 - 0.13170898365871023197e1_f64 * t14551 + t14554 - 0.79025390195226139182e1_f64 * t19196 - 0.79025390195226139182e1_f64 * t19199 - 0.13170898365871023197e1_f64 * t151 * t1251 * t6069 - t14556 - 0.13170898365871023197e1_f64 * t24410 + 0.79025390195226139182e1_f64 * t19208 + 0.79025390195226139182e1_f64 * t14564 - 0.13170898365871023197e1_f64 * t19213;
    t24415
}
