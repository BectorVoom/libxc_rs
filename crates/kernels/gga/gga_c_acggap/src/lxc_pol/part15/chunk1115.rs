//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1115/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1115<F: Float>(t10018: F, t159: F, t1658: F, t2146: F, t2147: F, t2222: F, t2385: F, t2394: F, t2400: F, t33250: F, t33566: F, t38361: F, t38370: F, t38377: F, t38379: F, t38382: F, t40601: F, t40709: F, t42181: F, t616: F, t619: F, t639: F, t6438: F, t7912: F, t7931: F, t8004: F, t8306: F, t9003: F, t9136: F) -> (F,) {
    let t42189 = 0.17347256376410398924e1 * t2146 * t2147 * t2385 * t1658 - 0.39512695097613069591e1 * t2222 * t6438 - 0.52041769129231196772e1 * t2146 * t8004 * t2394 * t1658 + 0.13170898365871023197e1 * t38361 - 0.13170898365871023197e1 * t33250 - 0.4336814094102599731e0 * t40601 * t639 + 0.8673628188205199462e0 * t7912 * t10018 + 0.8673628188205199462e0 * t9003 * t9136 + 0.8673628188205199462e0 * t33566 * t2400 + t38370 - 0.4336814094102599731e0 * t616 * t619 * t159 * t42181 + t38377 - t38379 - 0.8673628188205199462e0 * t7931 * t8306 * t40709 - t38382;
    (t42189,)
}
