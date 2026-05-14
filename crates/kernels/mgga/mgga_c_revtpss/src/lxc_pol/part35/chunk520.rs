//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 520/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk520<F: Float>(t45: F, t4303: F, t4306: F, t2498: F, t2518: F, t2522: F, t2562: F, t2569: F, t2579: F, t2587: F, t2610: F, t2628: F, t2632: F, t4397: F, t2375: F, t5819: F, t5825: F, t78: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t5924 = 0.36622894612013090108e-3 * t4303;
    let t5925 = 8.0 * t4306;
    let t5926 = -t2569 + t2579 + t2587 - t2522 - t2498 - t2518 + t2610 - t5924 - t2562 + t5925 + t2632 + t2628;
    let t5927 = 2.0 * t4397;
    let t5933 = piecewise3(t151, 0.0, 4.0 / 9.0 * t2375 * t5819 + 4.0 / 3.0 * t78 * t5825);
    (t5924, t5925, t5926, t5927, t5933)
}
