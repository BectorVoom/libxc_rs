//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1042/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1042<F: Float>(t7760: F, t786: F, t789: F, t27198: F, t867: F, t7060: F, t7063: F, t14685: F, t1941: F, t25245: F, t4430: F, t1561: F, t25266: F, t1945: F, t4371: F, t807: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t27202 = t786 * t7760;
    let t27203 = t27202 * t789;
    let t27212 = t27198 * t867;
    let t27213 = t786 * t27212;
    let t27214 = t27213 * t7060;
    let t27216 = t7063 * t27212;
    let t27217 = t27216 * t7060;
    let t27221 = t1941 * t14685;
    let t27228 = t25245 * t4430;
    let t27230 = t25266 * t1561;
    let t27239 = t1945 * t4371;
    let t27240 = t807 * t27239;
    (t27202, t27203, t27212, t27213, t27214, t27216, t27217, t27221, t27228, t27230, t27239, t27240)
}
