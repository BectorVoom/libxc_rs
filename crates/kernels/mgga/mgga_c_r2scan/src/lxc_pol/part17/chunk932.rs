//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 932/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk932<F: Float>(t1567: F, t489: F, t146: F, t252: F, t108: F, t2214: F, t10979: F, t128: F, t409: F, t5: F, t511: F, t7: F, t2096: F, t4145: F, t133: F, t5052: F) -> (F, F, F, F, F, F, F) {
    let t20303 = t489 * t1567;
    let t20305 = t146 * t20303 * t252;
    let t20407 = t2214 * t108;
    let t20421 = t10979 * t128;
    let t20450 = t5 * t7 * t409 * t511;
    let t20544 = t2096 * t2096;
    let t20621 = t4145 * t128;
    let t20946 = t5052 * t133;
    (t20305, t20407, t20421, t20450, t20544, t20621, t20946)
}
