//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 613/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk613<F: Float>(t1280: F, t5230: F, t1287: F, t5346: F, t1774: F, t3759: F, t5245: F, t354: F, t471: F, t1214: F, t5351: F, t3766: F, t487: F) -> (F, F, F, F, F, F, F, F) {
    let t5443 = t1280 * t5230;
    let t5446 = t5346 * t1287;
    let t5449 = t3759 * t1774;
    let t5452 = t1280 * t5245;
    let t5457 = t354 * t471;
    let t5458 = t5457 * t1214;
    let t5459 = t5351 * t5458;
    let t5462 = t3766 * t487;
    (t5443, t5446, t5449, t5452, t5457, t5458, t5459, t5462)
}
