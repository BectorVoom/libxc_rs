//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1176/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1176<F: Float>(t1955: F, t6888: F, t225: F, t30055: F, t2022: F, t6861: F, t4003: F, t26079: F, t543: F, t7301: F, t6843: F, t1882: F, t7910: F) -> (F, F, F, F, F, F, F, F, F) {
    let t30071 = t1955 * t6888;
    let t30074 = t30055 * t225;
    let t30080 = t2022 * t6861;
    let t30081 = t30080 * t4003;
    let t30082 = t26079 * t30081;
    let t30088 = t30080 * t543;
    let t30089 = t7301 * t30088;
    let t30095 = t2022 * t6843 * t543;
    let t30096 = t7301 * t30095;
    let t30100 = t7910 * t1882 * t543;
    (t30071, t30074, t30081, t30082, t30088, t30089, t30095, t30096, t30100)
}
