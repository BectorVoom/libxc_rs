//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3464/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3464(t3151: f64, t6244: f64, t1024: f64, t11782: f64, t12132: f64, t12146: f64, t12154: f64, t15648: f64, t15655: f64, t16449: f64, t16515: f64, t16534: f64, t19492: f64, t19498: f64, t19549: f64, t19569: f64, t19572: f64, t20146: f64, t3043: f64, t3204: f64, t3304: f64, t3318: f64, t43378: f64, t43438: f64, t43450: f64, t43456: f64, t4757: f64, t4954: f64, t4981: f64, t5004: f64, t5005: f64, t55579: f64, t55583: f64, t6365: f64, t6371: f64, t6389: f64) -> (f64, f64) {
    let t65261 = t6244 * t3151;
    let t65279 = 0.79025390195226139182e1_f64 * t55579 * t19549 - 0.79025390195226139182e1_f64 * t55583 * t19492 - 0.13170898365871023197e1_f64 * t12154 * t19498 + 0.13170898365871023197e1_f64 * t4981 * t19572 * t12132 - 0.13170898365871023197e1_f64 * t1024 * t5004 * t15648 - 0.65854491829355115987e0_f64 * t11782 * t6371 + 0.13170898365871023197e1_f64 * t4954 * t16515 - 0.26341796731742046394e1_f64 * t19569 * t16534 + 0.65854491829355115987e0_f64 * t3043 * t6389 + 0.26341796731742046394e1_f64 * t43438 * t65261 * t3304 - 0.13170898365871023197e1_f64 * t43456 * t65261 * t3318 - 0.13170898365871023197e1_f64 * t43450 * t6365 - 0.26341796731742046394e1_f64 * t43378 * t6365 - 0.26341796731742046394e1_f64 * t12146 * t20146 - 0.26341796731742046394e1_f64 * t15655 * t5005 + 0.52683593463484092788e1_f64 * t3204 * t16449 * t4757;
    (t65261, t65279)
}
