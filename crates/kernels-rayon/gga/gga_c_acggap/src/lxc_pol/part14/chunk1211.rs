//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1211/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1211(t1915: f64, t32124: f64, t33535: f64, t33683: f64, t33686: f64, t33691: f64, t33695: f64, t33702: f64, t33715: f64, t38784: f64, t40698: f64, t40705: f64, t40709: f64, t40721: f64, t556: f64, t7931: f64, t7932: f64, t8001: f64, t8400: f64, t8428: f64, t8440: f64, t8791: f64, t9003: f64, t9033: f64) -> f64 {
    let t40729 = -0.52041769129231196772e1_f64 * t33683 - 0.8673628188205199462e0_f64 * t40698 + 0.4336814094102599731e0_f64 * t8400 * t7932 * t38784 - t33686 + t33691 + 0.8673628188205199462e0_f64 * t40705 - 0.26341796731742046394e1_f64 * t33695 + t33702 - 0.8673628188205199462e0_f64 * t7931 * t7932 * t40709 + 0.52041769129231196772e1_f64 * t32124 * t33535 * t8440 + 0.8673628188205199462e0_f64 * t9003 * t8428 + 0.26341796731742046394e1_f64 * t33715 + 0.8673628188205199462e0_f64 * t40721 + 0.13170898365871023197e1_f64 * t8001 * t1915 + 0.34694512752820797848e1_f64 * t7931 * t9033 * t556 * t8791;
    t40729
}
