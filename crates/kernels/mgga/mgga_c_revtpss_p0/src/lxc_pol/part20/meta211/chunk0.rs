//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 989/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk989<F: Float>(t10380: F, t38: F, t2851: F, t78: F, t2299: F, t606: F, t3361: F, t81: F, t2306: F, t10326: F, t10356: F, t2258: F, t633: F, t637: F) -> (F, F, F, F, F, F) {
    let t10381 = t38 * t10380;
    let t10389 = F::new(1.0) / t78 / t2851;
    let t10392 = t2299 * t606;
    let t10398 = F::new(1.0) / t81 / t3361;
    let t10401 = t2306 * t606;
    let t10406 = -F::new(280.0) / F::new(27.0) * t10389 * t10356 + F::new(28.0) / F::new(3.0) * t10392 * t2258 - F::new(4.0) / F::new(3.0) * t633 * t10326 + F::new(280.0) / F::new(27.0) * t10398 * t10356 + F::new(28.0) / F::new(3.0) * t10401 * t2258 + F::new(4.0) / F::new(3.0) * t637 * t10326;
    (t10381, t10389, t10392, t10398, t10401, t10406)
}
