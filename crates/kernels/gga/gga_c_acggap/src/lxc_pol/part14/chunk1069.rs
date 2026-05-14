//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1069/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1069<F: Float>(t315: F, t40619: F, t2134: F, t1839: F, t309: F, t7932: F, t7963: F, t157: F, t1937: F, t406: F, t2132: F, t2138: F, t322: F, t9767: F, t1915: F, t32124: F, t33535: F, t33683: F, t33686: F, t33691: F, t33695: F, t33702: F, t33715: F, t38784: F, t556: F, t7931: F, t8001: F, t8400: F, t8428: F, t8440: F, t8791: F, t9003: F, t9033: F) -> (F,) {
    let t40697 = t315 * t40619;
    let t40698 = t40697 * t2134;
    let t40703 = t1839 * t309;
    let t40705 = t7963 * t7932 * t40703;
    let t40709 = t1937 * t406 * t157;
    let t40721 = t2138 * t2132 * t9767 * t322;
    let t40729 = -0.52041769129231196772e1 * t33683 - 0.8673628188205199462e0 * t40698 + 0.4336814094102599731e0 * t8400 * t7932 * t38784 - t33686 + t33691 + 0.8673628188205199462e0 * t40705 - 0.26341796731742046394e1 * t33695 + t33702 - 0.8673628188205199462e0 * t7931 * t7932 * t40709 + 0.52041769129231196772e1 * t32124 * t33535 * t8440 + 0.8673628188205199462e0 * t9003 * t8428 + 0.26341796731742046394e1 * t33715 + 0.8673628188205199462e0 * t40721 + 0.13170898365871023197e1 * t8001 * t1915 + 0.34694512752820797848e1 * t7931 * t9033 * t556 * t8791;
    (t40729,)
}
