//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3565/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3565<F: Float>(t5015: F, t1076: F, t11120: F, t11214: F, t16249: F, t1652: F, t16603: F, t1696: F, t20191: F, t3058: F, t3066: F, t3076: F, t3261: F, t3269: F, t43656: F, t4758: F, t4778: F, t52994: F, t53281: F, t55461: F, t55475: F, t6235: F, t6245: F, t6251: F, t6350: F, t64912: F, t64989: F, t995: F, t996: F) -> F {
    let t68117 = t5015 * t5015;
    let t68130 = F::cast_from(0.26341796731742046394e1_f64) * t43656 * t6245 - F::cast_from(0.26341796731742046394e1_f64) * t55475 * t1652 + F::cast_from(0.79025390195226139182e1_f64) * t16603 * t11120 * t6350 * t3066 - F::cast_from(0.26341796731742046394e1_f64) * t52994 * t1696 + F::cast_from(0.65854491829355115987e0_f64) * t6235 * t3261 - F::cast_from(0.13170898365871023197e1_f64) * t20191 * t3076 + F::cast_from(0.52683593463484092788e1_f64) * t53281 * t4758 + F::cast_from(0.26341796731742046394e1_f64) * t3058 * t996 * t64912 + F::cast_from(0.26341796731742046394e1_f64) * t1076 * t3269 * t68117 - F::cast_from(0.13170898365871023197e1_f64) * t55461 * t1696 + F::cast_from(0.13170898365871023197e1_f64) * t11214 * t6251 - F::cast_from(0.65854491829355115987e0_f64) * t995 * t996 * t64989 - F::cast_from(0.26341796731742046394e1_f64) * t4778 * t16249;
    t68130
}
