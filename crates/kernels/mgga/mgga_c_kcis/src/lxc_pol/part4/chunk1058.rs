//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1058/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1058<F: Float>(t2851: F, t4999: F, t1020: F, t2822: F, t5000: F, t2856: F, t251: F, t691: F, t1018: F, t86: F, t4996: F, t2855: F, t4621: F) -> (F, F, F, F, F, F) {
    let t13388 = t4999 * t2851;
    let t13389 = t1020 * t13388;
    let t13391 = t2822 * t5000;
    let t13393 = t4999 * t2856;
    let t13394 = t1020 * t13393;
    let t13396 = t691 * t251;
    let t13398 = t86 * t13396 * t1018;
    let t13399 = t13398 * t4996;
    let t13401 = t2855 * t4621;
    (t13389, t13391, t13394, t13396, t13399, t13401)
}
