//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3792/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3792(t12641: f64, t13177: f64, t17974: f64, t17992: f64, t18030: f64, t18102: f64, t18114: f64, t1829: f64, t20700: f64, t20710: f64, t20714: f64, t20744: f64, t21408: f64, t3556: f64, t3729: f64, t3732: f64, t3791: f64, t5237: f64, t5417: f64, t56303: f64, t56327: f64, t56332: f64, t56432: f64, t6564: f64, t6588: f64) -> f64 {
    let t72925 = 0.13170898365871023197e1_f64 * t3556 * t20710 - 0.79025390195226139182e1_f64 * t56332 * t18030 - 0.13170898365871023197e1_f64 * t13177 * t6588 + 0.79025390195226139182e1_f64 * t56327 * t17974 * t18102 + 0.65854491829355115987e0_f64 * t6564 * t3729 + 0.26341796731742046394e1_f64 * t18114 * t5237 + 0.26341796731742046394e1_f64 * t5417 * t17992 - 0.65854491829355115987e0_f64 * t20700 * t3791 + 0.52683593463484092788e1_f64 * t3732 * t21408 - 0.26341796731742046394e1_f64 * t12641 * t20714 - 0.13170898365871023197e1_f64 * t56303 * t1829 - 0.52683593463484092788e1_f64 * t56432 * t20744;
    t72925
}
