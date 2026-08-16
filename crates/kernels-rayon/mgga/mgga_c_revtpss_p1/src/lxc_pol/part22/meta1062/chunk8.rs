//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3800/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3800(t1210: f64, t1271: f64, t1277: f64, t13177: f64, t16750: f64, t1774: f64, t1775: f64, t17963: f64, t17999: f64, t18037: f64, t18065: f64, t18084: f64, t1828: f64, t20728: f64, t20748: f64, t21333: f64, t3556: f64, t3572: f64, t45449: f64, t495: f64, t5220: f64, t5237: f64, t5251: f64, t5429: f64, t56413: f64, t6580: f64, t71179: f64) -> f64 {
    let t73177 = 0.65854491829355115987e0_f64 * t71179 * t495 + 0.13170898365871023197e1_f64 * t21333 * t1271 + 0.52683593463484092788e1_f64 * t18065 * t5429 + 0.13170898365871023197e1_f64 * t3556 * t20728 + 0.13170898365871023197e1_f64 * t1210 * t1277 * t16750 * t1828 + 0.13170898365871023197e1_f64 * t5220 * t18084 + 0.26341796731742046394e1_f64 * t18037 * t5237 + 0.13170898365871023197e1_f64 * t5251 * t17999 - 0.79025390195226139182e1_f64 * t45449 * t20748 + 0.13170898365871023197e1_f64 * t1210 * t1277 * t1774 * t17963 - 0.13170898365871023197e1_f64 * t56413 * t1775 + 0.13170898365871023197e1_f64 * t3572 * t20728 + 0.26341796731742046394e1_f64 * t13177 * t6580;
    t73177
}
