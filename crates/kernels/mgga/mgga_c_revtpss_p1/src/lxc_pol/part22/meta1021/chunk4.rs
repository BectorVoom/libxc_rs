//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3553/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3553<F: Float>(t12046: F, t1678: F, t342: F, t1086: F, t6343: F, t994: F, t4772: F, t4975: F, t12050: F, t19450: F, t1089: F, t12127: F, t12149: F, t16396: F, t16405: F, t16432: F, t16569: F, t16573: F, t16581: F, t19446: F, t19447: F, t19503: F, t19572: F, t19580: F, t19603: F, t19829: F, t20139: F, t3287: F, t3288: F, t43357: F, t43360: F, t43443: F, t43446: F, t43520: F, t43528: F, t43598: F, t4857: F, t4905: F, t4976: F, t4996: F, t6365: F, t65773: F, t66382: F, t67545: F, t73: F) -> (F, F) {
    let t67644 = t342 * t12046 * t1678;
    let t67652 = t994 * t1086 * t6343;
    let t67668 = t4975 * t4772;
    let t67678 = t19450 * t12050;
    let t67684 = F::cast_from(0.13170898365871023197e1_f64) * t43528 * t19580 - F::cast_from(0.13170898365871023197e1_f64) * t4857 * t16396 + F::cast_from(0.26341796731742046394e1_f64) * t43598 * t20139 + F::cast_from(0.13170898365871023197e1_f64) * t67644 * t16569 + F::cast_from(0.26341796731742046394e1_f64) * t19603 * t16581 - F::cast_from(0.13170898365871023197e1_f64) * t43357 * t6365 - F::cast_from(0.13170898365871023197e1_f64) * t67652 * t3288 - F::cast_from(0.65854491829355115987e0_f64) * t4996 * t19572 * t16573 - F::cast_from(0.26341796731742046394e1_f64) * t43360 * t19503 - F::cast_from(0.13170898365871023197e1_f64) * t3287 * t67545 * t1089 - F::cast_from(0.79025390195226139182e1_f64) * t43446 * t19829 * t73 * t4976 + F::cast_from(0.52683593463484092788e1_f64) * t12149 * t4905 * t73 * t67668 + F::cast_from(0.26341796731742046394e1_f64) * t12127 * t16432 * t65773 + F::cast_from(0.26341796731742046394e1_f64) * t12149 * t19446 * t16405 - F::cast_from(0.79025390195226139182e1_f64) * t43520 * t67678 * t66382 + F::cast_from(0.52683593463484092788e1_f64) * t43443 * t19447;
    (t67678, t67684)
}
