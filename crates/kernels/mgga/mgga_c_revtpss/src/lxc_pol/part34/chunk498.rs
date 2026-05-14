//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 498/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk498<F: Float>(t159: F, t550: F, t216: F, t1376: F, t2689: F, t2700: F, t535: F, t1369: F, t794: F, t2453: F, t546: F) -> (F, F, F, F, F, F) {
    let t3943 = t159 * t550;
    let t3944 = t216 * t3943;
    let t3950 = 0.76220476654346199061e-4 * t2689 * t1376;
    let t3956 = 35.0 / 432.0 * t2700 * t535;
    let t3957 = t794 * t1369;
    let t3964 = t2453 * t546;
    (t3943, t3944, t3950, t3956, t3957, t3964)
}
