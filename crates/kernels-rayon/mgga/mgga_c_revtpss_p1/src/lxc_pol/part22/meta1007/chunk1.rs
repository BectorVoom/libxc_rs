//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3444/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3444(t15669: f64, t379: f64, t11190: f64, t11224: f64, t16314: f64, t16328: f64, t16371: f64, t16597: f64, t1696: f64, t19381: f64, t19396: f64, t19415: f64, t19425: f64, t20172: f64, t3052: f64, t3063: f64, t3075: f64, t3269: f64, t4773: f64, t4778: f64, t4947: f64, t53093: f64, t6251: f64, t6350: f64, t995: f64) -> f64 {
    let t64711 = t15669 * t379;
    let t64722 = 0.52683593463484092788e1_f64 * t16371 * t4947 - 0.13170898365871023197e1_f64 * t995 * t3269 * t6350 * t3075 - 0.26341796731742046394e1_f64 * t16597 * t4773 + 0.26341796731742046394e1_f64 * t4778 * t16328 - 0.13170898365871023197e1_f64 * t53093 * t1696 + 0.13170898365871023197e1_f64 * t11190 * t6251 - 0.13170898365871023197e1_f64 * t3063 * t19381 - 0.52683593463484092788e1_f64 * t64711 * t16314 - 0.79025390195226139182e1_f64 * t3052 * t19425 + 0.26341796731742046394e1_f64 * t3063 * t19396 + 0.26341796731742046394e1_f64 * t3052 * t20172 + 0.26341796731742046394e1_f64 * t11224 * t19415;
    t64722
}
