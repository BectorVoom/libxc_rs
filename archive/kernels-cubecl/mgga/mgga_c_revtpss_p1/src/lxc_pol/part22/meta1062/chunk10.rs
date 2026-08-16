//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3802/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3802<F: Float>(t1204: F, t6695: F, t1276: F, t6573: F, t1211: F, t12628: F, t1295: F, t13181: F, t16771: F, t17968: F, t17986: F, t17995: F, t18019: F, t18073: F, t20710: F, t20760: F, t21390: F, t21624: F, t34934: F, t3572: F, t3575: F, t3732: F, t45430: F, t5225: F, t5251: F, t56327: F, t56419: F, t6574: F, t6702: F, t71606: F) -> F {
    let t73222 = t1204 * t6695;
    let t73236 = t1276 * t6573;
    let t73244 = F::cast_from(0.13170898365871023197e1_f64) * t45430 * t6574 - F::cast_from(0.52683593463484092788e1_f64) * t56419 * t21390 - F::cast_from(0.13170898365871023197e1_f64) * t3572 * t21624 - F::cast_from(0.39512695097613069591e1_f64) * t12628 * t1211 * t71606 + F::cast_from(0.26341796731742046394e1_f64) * t5251 * t18019 - F::cast_from(0.13170898365871023197e1_f64) * t73222 * t1295 - F::cast_from(0.15805078039045227836e2_f64) * t56327 * t34934 * t16771 + F::cast_from(0.79025390195226139182e1_f64) * t17986 * t13181 * t6702 * t3575 + F::cast_from(0.26341796731742046394e1_f64) * t17995 * t18073 - F::cast_from(0.79025390195226139182e1_f64) * t5225 * t17968 + F::cast_from(0.79025390195226139182e1_f64) * t56327 * t73236 * t3575 + F::cast_from(0.13170898365871023197e1_f64) * t3572 * t20710 + F::cast_from(0.26341796731742046394e1_f64) * t3732 * t20760;
    t73244
}
