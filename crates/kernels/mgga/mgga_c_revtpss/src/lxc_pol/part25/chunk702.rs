//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 702/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk702<F: Float>(t3782: F, t5330: F, t1248: F, t471: F, t3767: F, t3603: F, t1214: F, t1260: F, t3670: F, t3627: F, t3766: F, t487: F) -> (F, F, F, F, F, F, F, F) {
    let t5331 = t3782 * t5330;
    let t5333 = t1248 * t471;
    let t5340 = t3767 * t5330;
    let t5341 = t3603 * t1248;
    let t5352 = t471 * t1214;
    let t5384 = t3670 * t1260;
    let t5405 = t3627 * t471;
    let t5462 = t3766 * t487;
    (t5331, t5333, t5340, t5341, t5352, t5384, t5405, t5462)
}
