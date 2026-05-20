//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 874/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk874<F: Float>(t5940: F, t705: F, t2411: F, t6079: F, t5944: F, t750: F, t189: F, t5825: F, t212: F, t6041: F, t780: F, t689: F) -> (F, F, F, F, F) {
    let t18263 = t705 * t5940;
    let t18268 = t6079 * t2411;
    let t18301 = t5944 * t750;
    let t18305 = t189 * t5825;
    let t18316 = t212 * t6041;
    let t18317 = t18316 * t780;
    let t18318 = t689 * t18317;
    (t18263, t18268, t18301, t18305, t18318)
}
