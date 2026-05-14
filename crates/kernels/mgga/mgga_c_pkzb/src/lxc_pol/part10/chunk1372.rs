//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1372/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1372<F: Float>(t2197: F, t2234: F, t3766: F, t2198: F, t6142: F, t9859: F, t2240: F, t3765: F, t6199: F, t6201: F, t3073: F, t8003: F, t27406: F, t27408: F, t27411: F, t27414: F, t27417: F, t27420: F, t27423: F) -> (F, F, F, F, F, F) {
    let t27426 = 2.0 * t2197 * t3766 * t2234;
    let t27429 = 0.96491876992155210402e2 * t6142 * t9859 * t2198;
    let t27432 = 0.16081979498692535067e2 * t2240 * t9859 * t2234;
    let t27436 = 0.51726012919273400301e3 * t6199 * t3765 * t6201 * t2198;
    let t27439 = 0.32163958997385070134e2 * t2240 * t3073 * t8003;
    let t27440 = t27406 + t27408 + t27411 + t27414 - t27417 - t27420 - t27423 - t27426 - t27429 + t27432 + t27436 + t27439;
    (t27426, t27429, t27432, t27436, t27439, t27440)
}
