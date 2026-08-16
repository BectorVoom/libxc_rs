//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 995/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk995(t33698: f64, t33699: f64, t620: f64, t157: f64, t524: f64, t929: f64, t19834: f64, t2127: f64, t2351: f64, t29997: f64, t31991: f64, t31999: f64, t33672: f64, t33673: f64, t33675: f64, t33681: f64, t33683: f64, t33686: f64, t33691: f64, t33695: f64, t4119: f64, t7931: f64, t7932: f64, t7938: f64, t8400: f64, t8440: f64) -> (f64, f64) {
    let t33702 = 0.10408353825846239354e2_f64 * t33698 * t620 * t33699;
    let t33706 = t524 * t929 * t157;
    let t33711 = 0.8673628188205199462e0_f64 * t8400 * t7932 * t19834 + t33672 + 0.34694512752820797848e1_f64 * t7931 * t33673 * t33675 + t33681 - 0.26020884564615598386e1_f64 * t33683 - t33686 - 0.17347256376410398924e1_f64 * t7931 * t29997 * t8440 + t33691 + 0.26341796731742046394e1_f64 * t2127 * t4119 - 0.13170898365871023197e1_f64 * t33695 - 0.8673628188205199462e0_f64 * t31991 + t33702 - 0.4336814094102599731e0_f64 * t7938 * t2351 + 0.4336814094102599731e0_f64 * t8400 * t7932 * t33706 + 0.34694512752820797848e1_f64 * t31999;
    (t33706, t33711)
}
