//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 834/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk834<F: Float>(t3153: F, t6305: F, t359: F, t6343: F, t1086: F, t6235: F, t6299: F, t73: F, t1065: F, t6244: F, t3172: F, t6301: F) -> (F, F, F, F, F, F, F) {
    let t19501 = t6305 * t3153;
    let t19556 = t359 * t6343;
    let t19566 = t6235 * t1086;
    let t19572 = t6299 * t3153;
    let t19611 = t6299 * t73;
    let t19649 = t1065 * t6244;
    let t19658 = t3172 * t6301;
    (t19501, t19556, t19566, t19572, t19611, t19649, t19658)
}
