//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 465/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk465<F: Float>(t2703: F, t802: F, t234: F, t2453: F, t595: F, t65: F, t235: F, t826: F, t232: F, t821: F) -> (F, F, F, F, F, F) {
    let t2704 = t2703 * t802;
    let t2710 = t2453 * t234;
    let t2712 = F::new(1.0) / t65 / t595;
    let t2713 = t235 * t2712;
    let t2716 = F::cast_from(0.45178982497454656791e-5_f64) * t2710 * t2713 * t826;
    let t2718 = F::new(1.0) / t821 / t232;
    (t2704, t2710, t2712, t2713, t2716, t2718)
}
