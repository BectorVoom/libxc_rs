//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 743/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk743<F: Float>(t2157: F, t5148: F, t5147: F, t1616: F, t560: F, t2201: F, t785: F, t481: F, t2207: F, t239: F, t4715: F, t5: F, t1398: F, t753: F, t1767: F, t2021: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5149 = t5148 * t2157;
    let t5150 = t5147 * t5149;
    let t5177 = t1616 * t560;
    let t5179 = t2201 * t785 * t5177;
    let t5181 = t1616 * t481;
    let t5183 = t2207 * t785 * t5181;
    let t5193 = 140.0 / 27.0 * t5 * t4715 * t239;
    let t5195 = t5 * t1398 * t753;
    let t5200 = t1767 * t2021;
    (t5149, t5150, t5177, t5179, t5181, t5183, t5193, t5195, t5200)
}
