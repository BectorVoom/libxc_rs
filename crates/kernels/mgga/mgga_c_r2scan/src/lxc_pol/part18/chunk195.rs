//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 195/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk195<F: Float>(t167: F, t585: F, t377: F, t5: F, t390: F) -> (F, F, F, F, F) {
    let t586 = t585 * t167;
    let t587 = t5 * t377;
    let t588 = t586 * t587;
    let t590 = F::cast_from(0.1046175e-1_f64) * t390;
    let t591 = -F::cast_from(0.14816666666666666667e-1_f64) * t588 - t590;
    (t586, t587, t588, t590, t591)
}
