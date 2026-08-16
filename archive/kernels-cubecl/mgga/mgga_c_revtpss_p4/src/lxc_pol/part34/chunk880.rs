//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 880/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk880<F: Float>(t18555: F, t757: F, t177: F, t5941: F, t762: F, t221: F, t2485: F, t6017: F, t2484: F, t125: F, t5962: F, t10779: F, t14671: F, t6035: F) -> (F, F, F, F, F, F) {
    let t18556 = t18555 * t757;
    let t18562 = t5941 * t177;
    let t18563 = t18562 * t762;
    let t18622 = t2485 * t221 * t6017;
    let t18623 = t2484 * t18622;
    let t18627 = t125 * t5962;
    let t18643 = t10779 * t14671 * t6035;
    (t18556, t18563, t18622, t18623, t18627, t18643)
}
