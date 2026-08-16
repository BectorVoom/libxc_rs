//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2245/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2245(t3596: f64, t8190: f64, t1214: f64, t1248: f64, t1287: f64, t1769: f64, t1774: f64, t1775: f64, t17992: f64, t18103: f64, t26884: f64, t26906: f64, t26922: f64, t26949: f64, t26976: f64, t29122: f64, t29158: f64, t29175: f64, t29178: f64, t29204: f64, t29220: f64, t29278: f64, t29304: f64, t3576: f64, t3588: f64, t3601: f64, t3769: f64, t3790: f64, t5457: f64, t7632: f64, t7636: f64, t7637: f64, t7643: f64, t7651: f64, t7652: f64, t7659: f64, t8213: f64, t96910: f64, t97419: f64) -> f64 {
    let t105090 = t3596 * t8190;
    let t105107 = -0.17347256376410398924e1_f64 * t29204 * t29175 + 0.8673628188205199462e0_f64 * t7643 * t7637 * t26884 * t1774 + 0.17347256376410398924e1_f64 * t26922 * t29178 * t1248 * t1287 - 0.13170898365871023197e1_f64 * t26976 * t18103 - 0.8673628188205199462e0_f64 * t7636 * t7637 * t26884 * t1769 + 0.13170898365871023197e1_f64 * t7632 * t17992 - 0.65854491829355115987e0_f64 * t97419 * t1775 + 0.13170898365871023197e1_f64 * t29220 * t3576 + 0.8673628188205199462e0_f64 * t26922 * t29158 * t5457 * t3790 - 0.4336814094102599731e0_f64 * t7659 * t29122 * t3588 * t1287 - 0.8673628188205199462e0_f64 * t26906 * t105090 * t3601 * t3769 + 0.13170898365871023197e1_f64 * t29304 * t3576 - 0.52041769129231196772e1_f64 * t26949 * t7637 * t29278 * t1214 + 0.8673628188205199462e0_f64 * t7651 * t7652 * t8190 * t3790 - 0.4336814094102599731e0_f64 * t96910 * t8213;
    t105107
}
