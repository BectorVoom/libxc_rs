//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 484/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk484<F: Float>(t1564: F, t379: F, t5675: F, t5674: F, t1800: F, t5635: F, t1317: F, t28: F, t469: F, t5617: F, t1322: F, t375: F, t89: F, t1307: F, t358: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5677 = t1564 * t5675 * t379;
    let t5678 = t5674 * t5677;
    let t5680 = t1800 * t5635;
    let t5682 = t1317 * t28 * t5680;
    let t5684 = t469 * t5617;
    let t5686 = t1317 * t28 * t5684;
    let t5689 = t89 * t375 * t1322;
    let t5690 = t5689 / 9.0;
    let t5691 = t1307 * t358;
    (t5677, t5678, t5680, t5682, t5684, t5686, t5689, t5690, t5691)
}
