//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2045/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2045<F: Float>(t359: F, t42066: F, t3143: F, t36870: F, t1983: F, t1981: F, t42058: F, t7143: F, t1982: F, t93484: F, t11120: F, t3140: F) -> (F, F, F, F, F) {
    let t93968 = t42066 * t359;
    let t93982 = t36870 * t3143;
    let t93983 = t1983 * t93982;
    let t93994 = t1981 * t42058 * t7143;
    let t94005 = t1982 * t93484;
    let t94014 = t3140 * t11120;
    (t93968, t93983, t93994, t94005, t94014)
}
