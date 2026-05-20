//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1754/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1754<F: Float>(t1561: F, t25266: F, t25270: F, t4462: F, t4447: F, t4452: F, t1945: F, t4371: F, t807: F, t4458: F, t7025: F, t1549: F, t25277: F) -> (F, F, F, F, F, F, F, F) {
    let t27230 = t25266 * t1561;
    let t27232 = t25270 * t4462;
    let t27234 = t25270 * t4447;
    let t27236 = t25270 * t4452;
    let t27239 = t1945 * t4371;
    let t27240 = t807 * t27239;
    let t27244 = t7025 * t4458;
    let t27246 = t25277 * t1549;
    (t27230, t27232, t27234, t27236, t27239, t27240, t27244, t27246)
}
