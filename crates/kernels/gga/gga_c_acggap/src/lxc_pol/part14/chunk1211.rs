//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1211/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1211<F: Float>(t1915: F, t32124: F, t33535: F, t33683: F, t33686: F, t33691: F, t33695: F, t33702: F, t33715: F, t38784: F, t40698: F, t40705: F, t40709: F, t40721: F, t556: F, t7931: F, t7932: F, t8001: F, t8400: F, t8428: F, t8440: F, t8791: F, t9003: F, t9033: F) -> F {
    let t40729 = -F::new(0.52041769129231196772e1) * t33683 - F::new(0.8673628188205199462e0) * t40698 + F::new(0.4336814094102599731e0) * t8400 * t7932 * t38784 - t33686 + t33691 + F::new(0.8673628188205199462e0) * t40705 - F::new(0.26341796731742046394e1) * t33695 + t33702 - F::new(0.8673628188205199462e0) * t7931 * t7932 * t40709 + F::new(0.52041769129231196772e1) * t32124 * t33535 * t8440 + F::new(0.8673628188205199462e0) * t9003 * t8428 + F::new(0.26341796731742046394e1) * t33715 + F::new(0.8673628188205199462e0) * t40721 + F::new(0.13170898365871023197e1) * t8001 * t1915 + F::new(0.34694512752820797848e1) * t7931 * t9033 * t556 * t8791;
    t40729
}
