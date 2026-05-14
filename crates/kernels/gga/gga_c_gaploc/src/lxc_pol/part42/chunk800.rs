//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 800/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk800<F: Float>(t42202: F, t42226: F, t13386: F, t1429: F, t549: F, t13261: F, t4614: F, t597: F, t10348: F, t3566: F, t11386: F, t2441: F, t13402: F, t587: F, t589: F, t13403: F, t1407: F) -> (F, F, F, F, F, F, F, F) {
    let t46605 = 0.25561950635947166451e0 * t42202;
    let t46606 = 0.23005755572352449806e1 * t42226;
    let t46608 = t1429 * t549 * t13386;
    let t46609 = 0.29792074959875355558e-1 * t46608;
    let t46612 = 0.15337170381568299871e2 * t597 * t4614 * t13261;
    let t46614 = 0.16683561977530199113e1 * t3566 * t10348;
    let t46630 = 0.35750489951850426669e0 * t2441 * t11386;
    let t46632 = t587 * t589 * t13402;
    let t46633 = 0.25561950635947166451e0 * t46632;
    let t46634 = t1407 * t13403;
    (t46605, t46606, t46609, t46612, t46614, t46630, t46633, t46634)
}
