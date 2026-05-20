//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3556/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3556<F: Float>(t3316: F, t6235: F, t1071: F, t1087: F, t1089: F, t12122: F, t12127: F, t12132: F, t12146: F, t16390: F, t16433: F, t16436: F, t16502: F, t19463: F, t19477: F, t19482: F, t19498: F, t19501: F, t19593: F, t19612: F, t3059: F, t3075: F, t3283: F, t3287: F, t3292: F, t3318: F, t3319: F, t378: F, t43456: F, t43611: F, t4976: F, t55732: F, t56049: F, t6386: F, t65425: F, t66341: F, t66395: F, t67501: F) -> F {
    let t67790 = t6235 * t3316;
    let t67813 = F::cast_from(0.65854491829355115987e0_f64) * t12127 * t19501 * t19482 * t3075 + F::cast_from(0.13170898365871023197e1_f64) * t1087 * t1071 * t19477 * t1089 - F::cast_from(0.26341796731742046394e1_f64) * t16502 * t16390 - F::cast_from(0.13170898365871023197e1_f64) * t43456 * t19501 * t19482 * t3059 - F::cast_from(0.26341796731742046394e1_f64) * t12122 * t19593 * t12132 - F::cast_from(0.52683593463484092788e1_f64) * t56049 * t16433 + F::cast_from(0.26341796731742046394e1_f64) * t55732 * t16436 - F::cast_from(0.65854491829355115987e0_f64) * t67790 * t3319 - F::cast_from(0.13170898365871023197e1_f64) * t19463 * t3292 + F::cast_from(0.13170898365871023197e1_f64) * t67501 * t3283 + F::cast_from(0.65854491829355115987e0_f64) * t12127 * t66341 * t3318 - F::cast_from(0.65854491829355115987e0_f64) * t43611 * t6386 - F::cast_from(0.13170898365871023197e1_f64) * t3287 * t66395 * t4976 - F::cast_from(0.13170898365871023197e1_f64) * t12146 * t19498 - F::cast_from(0.13170898365871023197e1_f64) * t12146 * t19612 + F::cast_from(0.65854491829355115987e0_f64) * t1087 * t378 * t65425 * t1089;
    t67813
}
