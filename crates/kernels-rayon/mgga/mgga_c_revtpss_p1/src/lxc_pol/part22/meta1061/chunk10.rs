//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3790/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3790(t13181: f64, t1774: f64, t1214: f64, t5428: f64, t12633: f64, t1277: f64, t17967: f64, t17973: f64, t17986: f64, t18005: f64, t18043: f64, t18047: f64, t18059: f64, t18070: f64, t18073: f64, t1829: f64, t20704: f64, t21348: f64, t21366: f64, t21382: f64, t21389: f64, t3567: f64, t3568: f64, t3572: f64, t3732: f64, t5220: f64, t5251: f64, t5429: f64, t56486: f64, t6744: f64) -> f64 {
    let t72843 = t13181 * t1774;
    let t72861 = t5428 * t1214;
    let t72865 = -0.13170898365871023197e1_f64 * t56486 * t1829 - 0.26341796731742046394e1_f64 * t5220 * t18047 + 0.52683593463484092788e1_f64 * t18005 * t5429 + 0.52683593463484092788e1_f64 * t18059 * t18070 + 0.26341796731742046394e1_f64 * t18059 * t18073 + 0.79025390195226139182e1_f64 * t17986 * t72843 * t17967 + 0.26341796731742046394e1_f64 * t3572 * t21382 - 0.79025390195226139182e1_f64 * t3732 * t21348 + 0.26341796731742046394e1_f64 * t12633 * t20704 + 0.26341796731742046394e1_f64 * t3572 * t21366 + 0.26341796731742046394e1_f64 * t5251 * t18043 - 0.13170898365871023197e1_f64 * t3567 * t1277 * t6744 * t3568 + 0.10536718692696818558e2_f64 * t17973 * t21389 * t72861;
    t72865
}
