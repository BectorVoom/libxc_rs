//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1334/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1334<F: Float>(t1907: F, t6922: F, t28196: F, t28197: F, t29589: F, t7898: F, t30005: F, t4248: F, t651: F, t6765: F, t7741: F, t1868: F, t6781: F) -> (F, F, F, F, F) {
    let t114780 = t1907 * t6922;
    let t114783 = F::cast_from(6.0_f64) * t28196 * t28197 * t114780;
    let t114785 = F::cast_from(3.0_f64) * t7898 * t29589;
    let t114787 = F::cast_from(6.0_f64) * t4248 * t30005;
    let t114790 = F::cast_from(6.0_f64) * t651 * t6765 * t7741;
    let t114791 = t1868 * t6781;
    (t114783, t114785, t114787, t114790, t114791)
}
