//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 810/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk810<F: Float>(t255: F, t571: F, t8196: F, t2086: F, t980: F, t2627: F, t6518: F, t2605: F, t5100: F, t1604: F, t8071: F, t6086: F, t7624: F) -> (F, F, F, F, F, F) {
    let t8198 = t571 * t8196 * t255;
    let t8201 = t980 * t2086;
    let t8224 = F::new(0.76830240467580968652e0) * t6518 * t2627;
    let t8227 = t5100 * t2605;
    let t8231 = F::new(0.54878743191129263322e-2) * t1604 * t8071;
    let t8232 = t6086 * t7624;
    (t8198, t8201, t8224, t8227, t8231, t8232)
}
