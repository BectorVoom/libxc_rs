//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3464/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3464<F: Float>(t3151: F, t6244: F, t1024: F, t11782: F, t12132: F, t12146: F, t12154: F, t15648: F, t15655: F, t16449: F, t16515: F, t16534: F, t19492: F, t19498: F, t19549: F, t19569: F, t19572: F, t20146: F, t3043: F, t3204: F, t3304: F, t3318: F, t43378: F, t43438: F, t43450: F, t43456: F, t4757: F, t4954: F, t4981: F, t5004: F, t5005: F, t55579: F, t55583: F, t6365: F, t6371: F, t6389: F) -> (F, F) {
    let t65261 = t6244 * t3151;
    let t65279 = F::cast_from(0.79025390195226139182e1_f64) * t55579 * t19549 - F::cast_from(0.79025390195226139182e1_f64) * t55583 * t19492 - F::cast_from(0.13170898365871023197e1_f64) * t12154 * t19498 + F::cast_from(0.13170898365871023197e1_f64) * t4981 * t19572 * t12132 - F::cast_from(0.13170898365871023197e1_f64) * t1024 * t5004 * t15648 - F::cast_from(0.65854491829355115987e0_f64) * t11782 * t6371 + F::cast_from(0.13170898365871023197e1_f64) * t4954 * t16515 - F::cast_from(0.26341796731742046394e1_f64) * t19569 * t16534 + F::cast_from(0.65854491829355115987e0_f64) * t3043 * t6389 + F::cast_from(0.26341796731742046394e1_f64) * t43438 * t65261 * t3304 - F::cast_from(0.13170898365871023197e1_f64) * t43456 * t65261 * t3318 - F::cast_from(0.13170898365871023197e1_f64) * t43450 * t6365 - F::cast_from(0.26341796731742046394e1_f64) * t43378 * t6365 - F::cast_from(0.26341796731742046394e1_f64) * t12146 * t20146 - F::cast_from(0.26341796731742046394e1_f64) * t15655 * t5005 + F::cast_from(0.52683593463484092788e1_f64) * t3204 * t16449 * t4757;
    (t65261, t65279)
}
