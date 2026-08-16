//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3445/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3445(t20112: f64, t994: f64, t1000: f64, t1079: f64, t11187: f64, t15579: f64, t16254: f64, t16312: f64, t16322: f64, t16374: f64, t16597: f64, t16603: f64, t19421: f64, t19428: f64, t20172: f64, t20195: f64, t3052: f64, t3075: f64, t3264: f64, t4743: f64, t4747: f64, t4764: f64, t4932: f64, t4935: f64, t4940: f64, t53130: f64, t6392: f64, t995: f64) -> f64 {
    let t64737 = t994 * t20112;
    let t64753 = -0.26341796731742046394e1_f64 * t11187 * t19421 + 0.26341796731742046394e1_f64 * t16597 * t4764 - 0.79025390195226139182e1_f64 * t4935 * t16322 + 0.65854491829355115987e0_f64 * t995 * t1079 * t6392 * t3075 - 0.52683593463484092788e1_f64 * t16312 * t53130 * t4940 - 0.13170898365871023197e1_f64 * t64737 * t1000 + 0.26341796731742046394e1_f64 * t16374 * t4764 - 0.52683593463484092788e1_f64 * t16603 * t19428 * t16254 + 0.52683593463484092788e1_f64 * t3052 * t20195 + 0.26341796731742046394e1_f64 * t3264 * t20172 + 0.13170898365871023197e1_f64 * t4747 * t15579 + 0.26341796731742046394e1_f64 * t4743 * t4932;
    t64753
}
