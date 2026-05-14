//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 891/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk891<F: Float>(t2331: F, t323: F, t851: F, t2137: F, t32123: F, t1619: F, t322: F, t620: F, t157: F, t524: F, t929: F, t19834: F, t2127: F, t2351: F, t29997: F, t31991: F, t31999: F, t33672: F, t33673: F, t33675: F, t33681: F, t33683: F, t33686: F, t33691: F, t4119: F, t7931: F, t7932: F, t7938: F, t8400: F, t8440: F) -> (F, F) {
    let t33695 = t851 * t2331 * t323;
    let t33698 = t2137 * t32123;
    let t33699 = t1619 * t322;
    let t33702 = 0.10408353825846239354e2 * t33698 * t620 * t33699;
    let t33706 = t524 * t929 * t157;
    let t33711 = 0.8673628188205199462e0 * t8400 * t7932 * t19834 + t33672 + 0.34694512752820797848e1 * t7931 * t33673 * t33675 + t33681 - 0.26020884564615598386e1 * t33683 - t33686 - 0.17347256376410398924e1 * t7931 * t29997 * t8440 + t33691 + 0.26341796731742046394e1 * t2127 * t4119 - 0.13170898365871023197e1 * t33695 - 0.8673628188205199462e0 * t31991 + t33702 - 0.4336814094102599731e0 * t7938 * t2351 + 0.4336814094102599731e0 * t8400 * t7932 * t33706 + 0.34694512752820797848e1 * t31999;
    (t33706, t33711)
}
