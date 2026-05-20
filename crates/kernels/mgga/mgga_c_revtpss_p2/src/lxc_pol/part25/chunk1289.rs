//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1289/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1289<F: Float>(t93484: F, t994: F, t1071: F, t7150: F, t8521: F, t359: F, t42066: F, t1043: F, t7135: F, t73: F, t3143: F, t36870: F) -> (F, F, F, F, F) {
    let t93959 = t994 * t93484;
    let t93962 = t7150 * t1071;
    let t93963 = t93962 * t8521;
    let t93968 = t42066 * t359;
    let t93974 = t7135 * t1043 * t73;
    let t93982 = t36870 * t3143;
    (t93959, t93963, t93968, t93974, t93982)
}
