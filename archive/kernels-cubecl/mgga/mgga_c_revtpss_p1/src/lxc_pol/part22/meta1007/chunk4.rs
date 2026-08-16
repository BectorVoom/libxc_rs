//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3447/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3447<F: Float>(t19855: F, t993: F, t378: F, t1000: F, t11190: F, t16305: F, t16340: F, t16362: F, t16374: F, t19425: F, t20175: F, t20195: F, t20211: F, t3058: F, t3067: F, t3264: F, t3269: F, t3270: F, t3271: F, t4773: F, t5016: F, t6244: F, t6258: F, t6259: F, t995: F) -> (F, F) {
    let t64816 = t19855 * t993;
    let t64817 = t64816 * t378;
    let t64822 = -F::cast_from(0.26341796731742046394e1_f64) * t16340 * t5016 - F::cast_from(0.13170898365871023197e1_f64) * t995 * t3269 * t6258 * t3270 - F::cast_from(0.26341796731742046394e1_f64) * t16374 * t4773 - F::cast_from(0.79025390195226139182e1_f64) * t3264 * t19425 - F::cast_from(0.26341796731742046394e1_f64) * t16305 * t4773 + F::cast_from(0.26341796731742046394e1_f64) * t3058 * t3269 * t6244 * t3270 + F::cast_from(0.52683593463484092788e1_f64) * t3264 * t20195 + F::cast_from(0.26341796731742046394e1_f64) * t20175 * t3271 - F::cast_from(0.26341796731742046394e1_f64) * t16362 * t5016 + F::cast_from(0.13170898365871023197e1_f64) * t20211 * t3067 - F::cast_from(0.13170898365871023197e1_f64) * t64817 * t1000 - F::cast_from(0.65854491829355115987e0_f64) * t11190 * t6259;
    (t64816, t64822)
}
