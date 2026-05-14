//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 956/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk956<F: Float>(t27198: F, t867: F, t786: F, t7063: F, t14685: F, t1941: F, t25245: F, t4430: F, t1561: F, t25266: F, t1945: F, t4371: F, t807: F, t1549: F, t25277: F, t25234: F, t4349: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t27212 = t27198 * t867;
    let t27213 = t786 * t27212;
    let t27216 = t7063 * t27212;
    let t27221 = t1941 * t14685;
    let t27228 = t25245 * t4430;
    let t27230 = t25266 * t1561;
    let t27239 = t1945 * t4371;
    let t27240 = t807 * t27239;
    let t27246 = t25277 * t1549;
    let t27251 = t25234 * t4349;
    (t27212, t27213, t27216, t27221, t27228, t27230, t27239, t27240, t27246, t27251)
}
