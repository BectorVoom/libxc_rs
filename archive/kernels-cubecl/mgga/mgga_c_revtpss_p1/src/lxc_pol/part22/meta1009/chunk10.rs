//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3462/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3462<F: Float>(t1647: F, t16565: F, t3133: F, t6258: F, t16183: F, t1651: F, t1082: F, t1089: F, t12127: F, t16520: F, t16537: F, t16540: F, t16552: F, t16569: F, t16573: F, t16577: F, t16581: F, t19446: F, t19450: F, t19501: F, t19521: F, t19526: F, t19593: F, t20133: F, t3059: F, t3204: F, t3223: F, t3287: F, t43438: F, t43446: F, t43512: F, t43568: F, t4964: F, t4982: F, t55646: F, t55685: F, t55732: F, t56049: F, t6375: F, t64835: F) -> (F, F, F) {
    let t65181 = t1647 * t16565;
    let t65186 = t6258 * t3133;
    let t65192 = t1651 * t16183;
    let t65196 = F::cast_from(0.13170898365871023197e1_f64) * t12127 * t19593 * t16573 - F::cast_from(0.79025390195226139182e1_f64) * t43446 * t19446 * t16577 - F::cast_from(0.26341796731742046394e1_f64) * t56049 * t16537 + F::cast_from(0.13170898365871023197e1_f64) * t55732 * t16540 + F::cast_from(0.13170898365871023197e1_f64) * t3204 * t1082 * t64835 - F::cast_from(0.26341796731742046394e1_f64) * t3223 * t20133 + F::cast_from(0.13170898365871023197e1_f64) * t43512 * t6375 + F::cast_from(0.52683593463484092788e1_f64) * t16520 * t19521 + F::cast_from(0.26341796731742046394e1_f64) * t43438 * t19501 * t4982 * t3059 + F::cast_from(0.39512695097613069591e1_f64) * t16552 * t19450 * t43568 - F::cast_from(0.26341796731742046394e1_f64) * t55646 * t4964 + F::cast_from(0.13170898365871023197e1_f64) * t65181 * t16569 + F::cast_from(0.26341796731742046394e1_f64) * t19526 * t16581 - F::cast_from(0.65854491829355115987e0_f64) * t3287 * t65186 * t1089 - F::cast_from(0.26341796731742046394e1_f64) * t55685 * t4964 - F::cast_from(0.13170898365871023197e1_f64) * t3287 * t65192 * t1089;
    (t65186, t65192, t65196)
}
