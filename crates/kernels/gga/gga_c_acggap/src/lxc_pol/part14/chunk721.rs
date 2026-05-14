//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 721/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk721<F: Float>(t5616: F, t604: F, t1181: F, t2068: F, t7380: F, t8544: F, t2095: F, t8505: F, t137: F, t1579: F, t336: F, t578: F, t7464: F, t7466: F, t7468: F, t7479: F, t7481: F, t7484: F, t7488: F, t7496: F, t7500: F, t7516: F, t7520: F) -> (F, F, F, F, F) {
    let t8738 = t604 * t5616;
    let t8739 = t1181 * t8738;
    let t8740 = t2068 * t8739;
    let t8742 = t7380 * t8544;
    let t8744 = t2095 * t8505;
    let t8747 = t336 * t1579 * t137;
    let t8748 = t578 * t8747;
    let t8750 = 0.18868855373762491241e-2 * t7464 - 0.28303283060643736861e-2 * t7466 + 0.7862023072401038017e-3 * t7468 + 0.52413487149340253445e-3 * t7479 - 0.31448092289604152068e-3 * t7481 + 0.22921875e-1 * t7484 + 0.1528125e-1 * t7488 - 0.7862023072401038017e-3 * t7496 + 0.31448092289604152068e-3 * t7500 + 0.31448092289604152068e-3 * t8740 + 0.22921875e-1 * t8742 + 0.1528125e-1 * t8744 + t7516 - t7520 - 0.38203125e-2 * t8748;
    (t8739, t8742, t8744, t8747, t8750)
}
