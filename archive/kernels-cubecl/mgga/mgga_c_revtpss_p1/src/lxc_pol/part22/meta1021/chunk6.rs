//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3555/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3555<F: Float>(t3298: F, t6235: F, t378: F, t65481: F, t11788: F, t12097: F, t12149: F, t12154: F, t16183: F, t16406: F, t16440: F, t16465: F, t16534: F, t16544: F, t19482: F, t19509: F, t19594: F, t19597: F, t19608: F, t19836: F, t20139: F, t20146: F, t3151: F, t3305: F, t3317: F, t3318: F, t43432: F, t43443: F, t43528: F, t4893: F, t4954: F, t4976: F, t4996: F, t55569: F, t55570: F, t55593: F, t55594: F, t6383: F, t64891: F, t73: F) -> (F, F) {
    let t67725 = t6235 * t3298;
    let t67748 = t378 * t65481;
    let t67768 = F::cast_from(0.13170898365871023197e1_f64) * t67725 * t3305 + F::cast_from(0.13170898365871023197e1_f64) * t4954 * t16440 - F::cast_from(0.26341796731742046394e1_f64) * t19608 * t16534 + F::cast_from(0.52683593463484092788e1_f64) * t11788 * t19509 + F::cast_from(0.13170898365871023197e1_f64) * t4954 * t16465 - F::cast_from(0.52683593463484092788e1_f64) * t43432 * t19594 + F::cast_from(0.26341796731742046394e1_f64) * t43528 * t19597 + F::cast_from(0.52683593463484092788e1_f64) * t12149 * t19836 * t73 * t4976 + F::cast_from(0.26341796731742046394e1_f64) * t43443 * t20139 + F::cast_from(0.65854491829355115987e0_f64) * t12097 * t6383 - F::cast_from(0.13170898365871023197e1_f64) * t3317 * t67748 * t3318 - F::cast_from(0.13170898365871023197e1_f64) * t4996 * t4893 * t19482 * t16183 + F::cast_from(0.15805078039045227836e2_f64) * t55593 * t64891 * t55594 * t3151 - F::cast_from(0.23707617058567841754e2_f64) * t55569 * t64891 * t55570 * t3151 - F::cast_from(0.13170898365871023197e1_f64) * t16544 * t16406 - F::cast_from(0.26341796731742046394e1_f64) * t12154 * t20146;
    (t67748, t67768)
}
