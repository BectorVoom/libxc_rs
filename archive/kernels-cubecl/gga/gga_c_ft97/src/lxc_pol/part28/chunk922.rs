//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 922/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk922<F: Float>(t108: F, t25846: F, t1339: F, t1786: F, t463: F, t5710: F, t1780: F, t8216: F, t1851: F, t5617: F, t1326: F, t370: F) -> (F, F, F, F, F, F, F) {
    let t101983 = t25846 * t108;
    let t102524 = t1786 * t1339;
    let t102678 = t463 * t5710;
    let t102682 = t1780 * t5710;
    let t102689 = t8216 * t1339;
    let t102724 = t1851 * t5617;
    let t102776 = t370 * t1326;
    (t101983, t102524, t102678, t102682, t102689, t102724, t102776)
}
