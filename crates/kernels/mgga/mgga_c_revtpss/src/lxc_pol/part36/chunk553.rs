//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 553/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk553<F: Float>(t5842: F, t2282: F, t5819: F, t5825: F, t60: F, t1480: F, t1483: F, t2290: F, t44: F, t56: F, t5835: F, t5838: F, t61: F, t38: F, t2299: F, t633: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t5843 = sigma2 * t5842;
    let t5848 = t2282 * t5819;
    let t5851 = t60 * t5825;
    let t5854 = 5.0 / 18.0 * t44 * t5835 + 5.0 / 6.0 * t44 * t5838 + 88.0 / 9.0 * t5843 * t61 + 40.0 / 9.0 * t1480 * t1483 + 5.0 / 18.0 * t56 * t5848 - 5.0 / 6.0 * t56 * t5851 - t2290;
    let t5855 = t38 * t5854;
    let t5860 = t2299 * t5819;
    let t5862 = t633 * t5825;
    (t5843, t5848, t5851, t5854, t5855, t5860, t5862)
}
