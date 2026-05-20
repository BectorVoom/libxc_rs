//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 588/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk588<F: Float>(t2880: F, t2881: F, t2846: F, t2848: F, t2855: F, t2860: F, t2864: F) -> (F, F) {
    let t2882 = t2880 * t2881;
    let t2884 = F::new(4.0) / F::new(9.0) * t2846;
    let t2889 = t2884 + F::new(2.0) / F::new(9.0) * t2848 - F::new(2.0) / F::new(9.0) * t2855 + F::new(2.0) / F::new(3.0) * t2860 - t2864 / F::new(3.0);
    (t2882, t2889)
}
