//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 695/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk695<F: Float>(t2046: F, t7297: F, t7393: F, t132: F, t26007: F, t271: F, t298: F, t34: F, t4766: F, t637: F, t71: F, t1223: F, t211: F, t1965: F, t1977: F, t1982: F) -> (F, F, F, F) {
    let t35487 = t2046 * t7297 * t7393;
    let t35496 = t26007 / t34 / t298 * t271 * t71 * t132 * t4766 * t637;
    let t35511 = t211 * t1223;
    let t35512 = t1965 * t35511;
    let t35514 = t1977 * t35512 * t1982;
    (t35487, t35496, t35512, t35514)
}
