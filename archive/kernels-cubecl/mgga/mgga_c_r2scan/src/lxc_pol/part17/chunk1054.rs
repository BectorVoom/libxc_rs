//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1054/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1054<F: Float>(t3438: F, t37355: F, t10978: F, t10979: F, t2317: F, t597: F, t10673: F, t10682: F, t2279: F, t357: F, t10647: F, t10652: F) -> (F, F, F, F, F, F) {
    let t37356 = t3438 * t37355;
    let t37358 = t10978 * t10979 * t2317 * t37356;
    let t37360 = t597 * t37355;
    let t37362 = t10673 * t10682 * t37360;
    let t37364 = t2279 * t357;
    let t37365 = t37364 * t10647;
    let t37366 = t37365 * t10652;
    (t37358, t37360, t37362, t37364, t37365, t37366)
}
