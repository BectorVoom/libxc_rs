//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3565/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3565(t5015: f64, t1076: f64, t11120: f64, t11214: f64, t16249: f64, t1652: f64, t16603: f64, t1696: f64, t20191: f64, t3058: f64, t3066: f64, t3076: f64, t3261: f64, t3269: f64, t43656: f64, t4758: f64, t4778: f64, t52994: f64, t53281: f64, t55461: f64, t55475: f64, t6235: f64, t6245: f64, t6251: f64, t6350: f64, t64912: f64, t64989: f64, t995: f64, t996: f64) -> f64 {
    let t68117 = t5015 * t5015;
    let t68130 = 0.26341796731742046394e1_f64 * t43656 * t6245 - 0.26341796731742046394e1_f64 * t55475 * t1652 + 0.79025390195226139182e1_f64 * t16603 * t11120 * t6350 * t3066 - 0.26341796731742046394e1_f64 * t52994 * t1696 + 0.65854491829355115987e0_f64 * t6235 * t3261 - 0.13170898365871023197e1_f64 * t20191 * t3076 + 0.52683593463484092788e1_f64 * t53281 * t4758 + 0.26341796731742046394e1_f64 * t3058 * t996 * t64912 + 0.26341796731742046394e1_f64 * t1076 * t3269 * t68117 - 0.13170898365871023197e1_f64 * t55461 * t1696 + 0.13170898365871023197e1_f64 * t11214 * t6251 - 0.65854491829355115987e0_f64 * t995 * t996 * t64989 - 0.26341796731742046394e1_f64 * t4778 * t16249;
    t68130
}
