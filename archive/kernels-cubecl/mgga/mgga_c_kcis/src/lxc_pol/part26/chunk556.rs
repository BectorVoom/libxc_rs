//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 556/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk556<F: Float>(t1396: F, t5477: F, t1395: F, t5780: F, t1951: F, t532: F, t833: F, t1409: F, t1650: F, t1419: F, t167: F, t518: F) -> (F, F, F, F, F, F, F, F) {
    let t5781 = t1396 * t5477;
    let t5782 = t1395 * t5781;
    let t5783 = t5780 * t5782;
    let t5787 = t532 * t1951;
    let t5789 = t1951 * t833;
    let t5792 = t1409 * t1650;
    let t5793 = t5792 * t1419;
    let t5796 = t518 * t167;
    (t5781, t5782, t5783, t5787, t5789, t5792, t5793, t5796)
}
