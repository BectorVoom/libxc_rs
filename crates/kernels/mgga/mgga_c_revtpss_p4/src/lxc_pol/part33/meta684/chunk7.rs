//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2258/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2258<F: Float>(t105203: F, t105558: F, t105644: F, t1203: F, t1204: F, t20704: F, t2142: F, t21617: F, t26969: F, t26976: F, t26999: F, t29167: F, t29204: F, t29227: F, t29275: F, t29297: F, t30752: F, t30767: F, t30842: F, t30867: F, t30883: F, t30907: F, t5216: F, t5429: F, t6574: F, t6580: F, t7636: F, t7651: F, t7652: F, t7666: F, t8192: F, t8209: F, t96866: F) -> F {
    let t112602 = -F::cast_from(0.8673628188205199462e0_f64) * t30883 * t7666 + F::cast_from(0.13170898365871023197e1_f64) * t5216 * t8192 + F::cast_from(0.13170898365871023197e1_f64) * t26976 * t20704 + F::cast_from(0.13170898365871023197e1_f64) * t26999 * t6580 - F::cast_from(0.8673628188205199462e0_f64) * t29204 * t30752 + F::cast_from(0.65854491829355115987e0_f64) * t1204 * t30842 - F::cast_from(0.52041769129231196772e1_f64) * t7636 * t26969 * t30767 * t1203 + F::cast_from(0.13170898365871023197e1_f64) * t96866 * t6574 + F::cast_from(0.8673628188205199462e0_f64) * t7651 * t7652 * t2142 * t21617 + F::cast_from(0.34694512752820797848e1_f64) * t29204 * t30907 + F::cast_from(0.26341796731742046394e1_f64) * t29227 * t5429 + F::cast_from(0.17347256376410398924e1_f64) * t105203 * t8209 + F::cast_from(0.34694512752820797848e1_f64) * t29275 * t29297 + F::cast_from(0.34694512752820797848e1_f64) * t105644 * t30867 + F::cast_from(0.17347256376410398924e1_f64) * t105558 * t29167;
    t112602
}
