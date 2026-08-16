//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3802/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3802(t1204: f64, t6695: f64, t1276: f64, t6573: f64, t1211: f64, t12628: f64, t1295: f64, t13181: f64, t16771: f64, t17968: f64, t17986: f64, t17995: f64, t18019: f64, t18073: f64, t20710: f64, t20760: f64, t21390: f64, t21624: f64, t34934: f64, t3572: f64, t3575: f64, t3732: f64, t45430: f64, t5225: f64, t5251: f64, t56327: f64, t56419: f64, t6574: f64, t6702: f64, t71606: f64) -> f64 {
    let t73222 = t1204 * t6695;
    let t73236 = t1276 * t6573;
    let t73244 = 0.13170898365871023197e1_f64 * t45430 * t6574 - 0.52683593463484092788e1_f64 * t56419 * t21390 - 0.13170898365871023197e1_f64 * t3572 * t21624 - 0.39512695097613069591e1_f64 * t12628 * t1211 * t71606 + 0.26341796731742046394e1_f64 * t5251 * t18019 - 0.13170898365871023197e1_f64 * t73222 * t1295 - 0.15805078039045227836e2_f64 * t56327 * t34934 * t16771 + 0.79025390195226139182e1_f64 * t17986 * t13181 * t6702 * t3575 + 0.26341796731742046394e1_f64 * t17995 * t18073 - 0.79025390195226139182e1_f64 * t5225 * t17968 + 0.79025390195226139182e1_f64 * t56327 * t73236 * t3575 + 0.13170898365871023197e1_f64 * t3572 * t20710 + 0.26341796731742046394e1_f64 * t3732 * t20760;
    t73244
}
