//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 735/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk735<F: Float>(t10240: F, t2358: F, t661: F, t2357: F, t2362: F, t10236: F, t108: F, t101: F, t10217: F, t10229: F, t10233: F, t10237: F, t105: F, t2344: F, t2351: F, t2354: F, t656: F, t659: F, t97: F) -> (F,) {
    let t10241 = 1.0 / t10240;
    let t10242 = t2358 * t661;
    let t10243 = t10241 * t10242;
    let t10246 = t2357 * t661;
    let t10247 = t10246 * t2362;
    let t10250 = -t10236;
    let t10251 = t108 * t10250;
    let t10254 = -440.0 / 27.0 * t10217 * t101 + 200.0 / 9.0 * t2344 * t659 - 50.0 / 9.0 * t656 * t2351 - 25.0 / 3.0 * t656 * t2354 - 10.0 / 27.0 * t97 * t10229 + 10.0 / 3.0 * t97 * t10233 + 5.0 / 3.0 * t97 * t10237 - 10.0 / 27.0 * t105 * t10243 + 10.0 / 3.0 * t105 * t10247 + 5.0 / 3.0 * t105 * t10251;
    (t10254,)
}
