//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2245/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2245<F: Float>(t3596: F, t8190: F, t1214: F, t1248: F, t1287: F, t1769: F, t1774: F, t1775: F, t17992: F, t18103: F, t26884: F, t26906: F, t26922: F, t26949: F, t26976: F, t29122: F, t29158: F, t29175: F, t29178: F, t29204: F, t29220: F, t29278: F, t29304: F, t3576: F, t3588: F, t3601: F, t3769: F, t3790: F, t5457: F, t7632: F, t7636: F, t7637: F, t7643: F, t7651: F, t7652: F, t7659: F, t8213: F, t96910: F, t97419: F) -> F {
    let t105090 = t3596 * t8190;
    let t105107 = -F::cast_from(0.17347256376410398924e1_f64) * t29204 * t29175 + F::cast_from(0.8673628188205199462e0_f64) * t7643 * t7637 * t26884 * t1774 + F::cast_from(0.17347256376410398924e1_f64) * t26922 * t29178 * t1248 * t1287 - F::cast_from(0.13170898365871023197e1_f64) * t26976 * t18103 - F::cast_from(0.8673628188205199462e0_f64) * t7636 * t7637 * t26884 * t1769 + F::cast_from(0.13170898365871023197e1_f64) * t7632 * t17992 - F::cast_from(0.65854491829355115987e0_f64) * t97419 * t1775 + F::cast_from(0.13170898365871023197e1_f64) * t29220 * t3576 + F::cast_from(0.8673628188205199462e0_f64) * t26922 * t29158 * t5457 * t3790 - F::cast_from(0.4336814094102599731e0_f64) * t7659 * t29122 * t3588 * t1287 - F::cast_from(0.8673628188205199462e0_f64) * t26906 * t105090 * t3601 * t3769 + F::cast_from(0.13170898365871023197e1_f64) * t29304 * t3576 - F::cast_from(0.52041769129231196772e1_f64) * t26949 * t7637 * t29278 * t1214 + F::cast_from(0.8673628188205199462e0_f64) * t7651 * t7652 * t8190 * t3790 - F::cast_from(0.4336814094102599731e0_f64) * t96910 * t8213;
    t105107
}
