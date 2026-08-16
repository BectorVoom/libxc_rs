//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3450/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3450<F: Float>(t1024: F, t12097: F, t12122: F, t12127: F, t12149: F, t15655: F, t15886: F, t16152: F, t16450: F, t16458: F, t16552: F, t16554: F, t16561: F, t1692: F, t19414: F, t19488: F, t19556: F, t20089: F, t3075: F, t3151: F, t3204: F, t3278: F, t3291: F, t43520: F, t43524: F, t4857: F, t4970: F, t4976: F, t4983: F, t4998: F, t5004: F, t55499: F, t55887: F, t55938: F, t55939: F, t6379: F, t64848: F, t64854: F, t64861: F, t64891: F, t73: F) -> F {
    let t64896 = F::cast_from(0.15805078039045227836e2_f64) * t16552 * t55499 * t64848 + F::cast_from(0.52683593463484092788e1_f64) * t55887 * t16458 - F::cast_from(0.52683593463484092788e1_f64) * t12122 * t64854 * t4983 + F::cast_from(0.26341796731742046394e1_f64) * t12127 * t64854 * t4998 - F::cast_from(0.79025390195226139182e1_f64) * t43520 * t64861 * t16554 + F::cast_from(0.79025390195226139182e1_f64) * t43524 * t64861 * t16561 - F::cast_from(0.65854491829355115987e0_f64) * t1024 * t19556 * t3075 - F::cast_from(0.26341796731742046394e1_f64) * t15655 * t4970 + F::cast_from(0.52683593463484092788e1_f64) * t3204 * t5004 * t16152 + F::cast_from(0.13170898365871023197e1_f64) * t12097 * t6379 + F::cast_from(0.13170898365871023197e1_f64) * t15886 * t1692 + F::cast_from(0.52683593463484092788e1_f64) * t12149 * t20089 * t73 * t4976 + F::cast_from(0.26341796731742046394e1_f64) * t3204 * t3291 * t19414 - F::cast_from(0.26341796731742046394e1_f64) * t4857 * t16450 + F::cast_from(0.13170898365871023197e1_f64) * t3278 * t19488 + F::cast_from(0.92196288561097162379e1_f64) * t55938 * t64891 * t55939 * t3151;
    t64896
}
