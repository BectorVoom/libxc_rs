//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 678/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk678<F: Float>(t2157: F, t5148: F, t5147: F, t1616: F, t560: F, t2201: F, t785: F, t481: F, t2207: F, t239: F, t4715: F, t5: F) -> (F, F, F, F) {
    let t5149 = t5148 * t2157;
    let t5150 = t5147 * t5149;
    let t5177 = t1616 * t560;
    let t5179 = t2201 * t785 * t5177;
    let t5181 = t1616 * t481;
    let t5183 = t2207 * t785 * t5181;
    let t5193 = F::cast_from(140.0_f64) / F::cast_from(27.0_f64) * t5 * t4715 * t239;
    (t5150, t5179, t5183, t5193)
}
