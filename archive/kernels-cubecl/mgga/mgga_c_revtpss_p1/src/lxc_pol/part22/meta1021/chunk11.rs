//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3560/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3560<F: Float>(t1647: F, t16551: F, t16558: F, t1087: F, t1089: F, t12116: F, t12122: F, t16381: F, t16393: F, t16443: F, t16502: F, t16520: F, t16523: F, t16529: F, t16555: F, t16562: F, t19438: F, t19501: F, t19526: F, t19539: F, t19573: F, t19576: F, t20113: F, t3075: F, t3223: F, t3259: F, t4866: F, t4930: F, t4977: F, t4982: F, t4988: F, t4992: F, t5009: F, t55934: F, t6299: F, t989: F) -> F {
    let t67969 = t1647 * t16551;
    let t67972 = t1647 * t16558;
    let t67989 = F::cast_from(0.26341796731742046394e1_f64) * t1087 * t4930 * t4866 * t1089 + F::cast_from(0.26341796731742046394e1_f64) * t16520 * t19573 - F::cast_from(0.13170898365871023197e1_f64) * t16523 * t19576 + F::cast_from(0.26341796731742046394e1_f64) * t16381 * t4988 + F::cast_from(0.26341796731742046394e1_f64) * t16381 * t5009 + F::cast_from(0.26341796731742046394e1_f64) * t12116 * t19539 + F::cast_from(0.52683593463484092788e1_f64) * t19526 * t16443 - F::cast_from(0.26341796731742046394e1_f64) * t55934 * t4977 - F::cast_from(0.13170898365871023197e1_f64) * t12122 * t19501 * t4982 * t3075 + F::cast_from(0.79025390195226139182e1_f64) * t67969 * t16555 - F::cast_from(0.79025390195226139182e1_f64) * t67972 * t16562 + F::cast_from(0.13170898365871023197e1_f64) * t1647 * t16529 + F::cast_from(0.13170898365871023197e1_f64) * t989 * t20113 - F::cast_from(0.13170898365871023197e1_f64) * t3223 * t19438 + F::cast_from(0.65854491829355115987e0_f64) * t1087 * t3259 * t6299 * t1089 - F::cast_from(0.13170898365871023197e1_f64) * t16502 * t16393 + F::cast_from(0.26341796731742046394e1_f64) * t16381 * t4992;
    t67989
}
