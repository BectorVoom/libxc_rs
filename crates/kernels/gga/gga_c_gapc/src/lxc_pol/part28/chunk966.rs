//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 966/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk966<F: Float>(t102: F, t2299: F, t2530: F, t6851: F, t768: F, t122: F, t6856: F, t786: F, t4: F, t103: F, t2232: F, t7062: F, t880: F, t4914: F, t572: F, t10408: F, t883: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23579 = t2530 * t102 * t2299;
    let t23608 = t768 * t6851;
    let t23609 = t23608 * t122;
    let t23612 = t6856 * M_PI * t786;
    let t23624 = t6856 * t4;
    let t23678 = t2232 * t103;
    let t23723 = t880 * t7062;
    let t23726 = t572 * t4914;
    let t24004 = t10408 * t883;
    (t23579, t23608, t23609, t23612, t23624, t23678, t23723, t23726, t24004)
}
