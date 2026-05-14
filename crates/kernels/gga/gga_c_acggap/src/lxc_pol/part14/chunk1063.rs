//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1063/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1063<F: Float>(t1773: F, t2060: F, t2061: F, t6388: F, t7450: F, t7815: F, t13299: F, t33952: F, t40440: F, t15386: F, t34823: F, t40066: F, t31879: F, t36381: F, t36383: F, t36386: F, t36389: F, t36391: F, t37994: F, t40553: F, t40557: F, t40561: F, t40565: F, t40567: F, t40569: F, t40573: F) -> (F,) {
    let t40576 = t2060 * t1773 * t2061;
    let t40579 = t7450 * t7815 * t6388;
    let t40584 = t33952 * t13299 * t40440;
    let t40587 = t34823 * t15386 * t40066;
    let t40589 = 0.18868855373762491241e-2 * t40553 + 0.18868855373762491241e-2 * t40557 + 0.18868855373762491241e-2 * t40561 + 0.12579236915841660827e-2 * t40565 + 0.18868855373762491241e-2 * t40567 - 0.916875e-1 * t40569 + t36381 - t40573 / 384.0 + t36383 + 0.7640625e-2 * t40576 + t40579 / 32.0 - 0.85748036236139473944e-3 * t31879 + 0.5590771962596293701e-2 * t36386 + t36389 + t36391 + t37994 + 0.31448092289604152068e-2 * t40584 + 0.18868855373762491241e-2 * t40587;
    (t40589,)
}
