//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 688/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk688<F: Float>(t638: F, t7292: F, t7305: F, t2046: F, t7297: F, t7393: F, t132: F, t26007: F, t271: F, t298: F, t34: F, t4766: F, t637: F, t71: F, t1223: F, t211: F) -> (F, F, F, F) {
    let t35484 = t638 * t7292 * t7305;
    let t35487 = t2046 * t7297 * t7393;
    let t35496 = t26007 / t34 / t298 * t271 * t71 * t132 * t4766 * t637;
    let t35497 = 0.63245127235888530833e-7 * t35496;
    let t35511 = t211 * t1223;
    (t35484, t35487, t35497, t35511)
}
