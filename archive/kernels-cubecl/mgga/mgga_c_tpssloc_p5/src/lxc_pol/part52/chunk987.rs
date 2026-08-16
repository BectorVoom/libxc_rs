//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 987/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk987<F: Float>(t25014: F, t4255: F, t16596: F, t22960: F, t1484: F, t606: F, t25: F, t4119: F, t7484: F, t794: F, t6562: F, t1887: F, t23056: F) -> (F, F, F, F, F, F) {
    let t25015 = t25014 * t4255;
    let t25021 = t22960 * t16596;
    let t25024 = t606 * t1484;
    let t25028 = t25 * t4119;
    let t25035 = t794 * t7484;
    let t25036 = t6562 * t25035;
    let t25038 = t23056 * t1887;
    (t25015, t25021, t25024, t25028, t25036, t25038)
}
