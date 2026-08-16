//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3557/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3557<F: Float>(t1086: F, t19856: F, t19836: F, t3153: F, t1024: F, t1089: F, t1090: F, t12073: F, t12122: F, t12127: F, t12149: F, t15670: F, t16461: F, t16482: F, t16537: F, t16540: F, t16568: F, t16577: F, t19611: F, t20128: F, t3223: F, t3287: F, t3299: F, t3304: F, t43341: F, t4857: F, t4977: F, t4981: F, t4983: F, t4998: F, t55646: F, t55988: F, t55991: F, t6258: F, t64861: F, t66341: F, t66565: F, t67438: F, t67748: F) -> F {
    let t67825 = t19856 * t1086;
    let t67828 = t19836 * t3153;
    let t67859 = -F::cast_from(0.26341796731742046394e1_f64) * t3287 * t67438 * t1089 + F::cast_from(0.52683593463484092788e1_f64) * t15670 * t16482 - F::cast_from(0.26341796731742046394e1_f64) * t3223 * t20128 - F::cast_from(0.26341796731742046394e1_f64) * t4857 * t16461 + F::cast_from(0.13170898365871023197e1_f64) * t67825 * t1090 - F::cast_from(0.52683593463484092788e1_f64) * t12122 * t67828 * t4983 + F::cast_from(0.26341796731742046394e1_f64) * t12127 * t67828 * t4998 + F::cast_from(0.26341796731742046394e1_f64) * t4981 * t66565 * t4983 - F::cast_from(0.26341796731742046394e1_f64) * t55988 * t16537 + F::cast_from(0.13170898365871023197e1_f64) * t55991 * t16540 - F::cast_from(0.26341796731742046394e1_f64) * t55646 * t4977 + F::cast_from(0.13170898365871023197e1_f64) * t12149 * t19611 * t16577 - F::cast_from(0.13170898365871023197e1_f64) * t12122 * t66341 * t3304 - F::cast_from(0.65854491829355115987e0_f64) * t1024 * t12073 * t6258 + F::cast_from(0.26341796731742046394e1_f64) * t3299 * t67748 * t3304 - F::cast_from(0.13170898365871023197e1_f64) * t43341 * t64861 * t16568;
    t67859
}
