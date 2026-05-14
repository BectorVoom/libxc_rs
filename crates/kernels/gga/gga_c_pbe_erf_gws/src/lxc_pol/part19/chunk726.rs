//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 726/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk726<F: Float>(t589: F, t597: F, t187: F, t190: F, t5044: F, t1251: F, t607: F, t177: F, t572: F, t191: F, t4939: F, t1660: F, t9: F, t1764: F, t22: F, t1878: F, t586: F) -> (F, F, F, F, F, F, F, F) {
    let t5219 = t589 * t597;
    let t5241 = 0.29629629629629629629e-1 * t190 * t5044 * t187;
    let t5256 = t1251 * t607;
    let t5263 = 1.0 / t177 / t572;
    let t5264 = t191 * t5263;
    let t5271 = 0.11197407407407407407e0 * t4939;
    let t5283 = t9 * t1660;
    let t5292 = 1.0 / t187 / t1764;
    let t5293 = t22 * t5292;
    let t5312 = t1878 * t586;
    (t5219, t5241, t5256, t5264, t5271, t5283, t5293, t5312)
}
