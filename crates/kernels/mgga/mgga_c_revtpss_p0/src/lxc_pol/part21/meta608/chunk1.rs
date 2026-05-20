//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2350/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2350<F: Float>(t10428: F, t2615: F, t2622: F, t9586: F, t2514: F, t2492: F) -> (F, F, F, F) {
    let t39858 = t10428 * t2615;
    let t39860 = t2622 * t9586;
    let t39871 = t2514 * t2514;
    let t39875 = t2492 * t2492;
    (t39858, t39860, t39871, t39875)
}
