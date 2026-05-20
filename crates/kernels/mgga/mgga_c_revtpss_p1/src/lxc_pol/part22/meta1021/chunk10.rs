//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3559/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3559<F: Float>(t16543: F, t4746: F, t1087: F, t1089: F, t11788: F, t12122: F, t12154: F, t16405: F, t16406: F, t16432: F, t16443: F, t16502: F, t16523: F, t16578: F, t19463: F, t19484: F, t19503: F, t19512: F, t19603: F, t19611: F, t20136: F, t3133: F, t3287: F, t3288: F, t3295: F, t3317: F, t3318: F, t43432: F, t4964: F, t4996: F, t4998: F, t55701: F, t55747: F, t55887: F, t6343: F, t65881: F, t66565: F, t67869: F) -> F {
    let t67927 = t4746 * t16543;
    let t67946 = -F::cast_from(0.65854491829355115987e0_f64) * t3287 * t19611 * t16405 + F::cast_from(0.26341796731742046394e1_f64) * t55747 * t16578 + F::cast_from(0.52683593463484092788e1_f64) * t19603 * t16443 + F::cast_from(0.65854491829355115987e0_f64) * t1087 * t6343 * t3133 * t1089 + F::cast_from(0.26341796731742046394e1_f64) * t11788 * t19512 - F::cast_from(0.65854491829355115987e0_f64) * t3317 * t67869 * t3318 - F::cast_from(0.26341796731742046394e1_f64) * t12154 * t20136 - F::cast_from(0.26341796731742046394e1_f64) * t55701 * t4964 - F::cast_from(0.26341796731742046394e1_f64) * t67927 * t3288 - F::cast_from(0.13170898365871023197e1_f64) * t16502 * t16406 + F::cast_from(0.26341796731742046394e1_f64) * t55887 * t16578 - F::cast_from(0.13170898365871023197e1_f64) * t4996 * t66565 * t4998 - F::cast_from(0.26341796731742046394e1_f64) * t43432 * t19503 - F::cast_from(0.26341796731742046394e1_f64) * t16523 * t19484 - F::cast_from(0.65854491829355115987e0_f64) * t19463 * t3295 - F::cast_from(0.52683593463484092788e1_f64) * t12122 * t16432 * t65881;
    t67946
}
