//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2258/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2258(t105203: f64, t105558: f64, t105644: f64, t1203: f64, t1204: f64, t20704: f64, t2142: f64, t21617: f64, t26969: f64, t26976: f64, t26999: f64, t29167: f64, t29204: f64, t29227: f64, t29275: f64, t29297: f64, t30752: f64, t30767: f64, t30842: f64, t30867: f64, t30883: f64, t30907: f64, t5216: f64, t5429: f64, t6574: f64, t6580: f64, t7636: f64, t7651: f64, t7652: f64, t7666: f64, t8192: f64, t8209: f64, t96866: f64) -> f64 {
    let t112602 = -0.8673628188205199462e0_f64 * t30883 * t7666 + 0.13170898365871023197e1_f64 * t5216 * t8192 + 0.13170898365871023197e1_f64 * t26976 * t20704 + 0.13170898365871023197e1_f64 * t26999 * t6580 - 0.8673628188205199462e0_f64 * t29204 * t30752 + 0.65854491829355115987e0_f64 * t1204 * t30842 - 0.52041769129231196772e1_f64 * t7636 * t26969 * t30767 * t1203 + 0.13170898365871023197e1_f64 * t96866 * t6574 + 0.8673628188205199462e0_f64 * t7651 * t7652 * t2142 * t21617 + 0.34694512752820797848e1_f64 * t29204 * t30907 + 0.26341796731742046394e1_f64 * t29227 * t5429 + 0.17347256376410398924e1_f64 * t105203 * t8209 + 0.34694512752820797848e1_f64 * t29275 * t29297 + 0.34694512752820797848e1_f64 * t105644 * t30867 + 0.17347256376410398924e1_f64 * t105558 * t29167;
    t112602
}
