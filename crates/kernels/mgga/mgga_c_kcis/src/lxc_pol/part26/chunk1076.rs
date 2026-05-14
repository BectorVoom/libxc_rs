//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1076/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1076<F: Float>(t1628: F, t23253: F, t286: F, t69: F, t4413: F, t7490: F, t1591: F, t23024: F, t4479: F, t7533: F, t1385: F, t60029: F, t1610: F, t6284: F, t167: F, t2104: F) -> (F, F, F, F, F, F, F, F) {
    let t60988 = t23253 * t1628;
    let t61287 = t69 * t286;
    let t62417 = t7490 * t4413;
    let t62923 = t23024 * t1591;
    let t63256 = t7533 * t4479;
    let t75638 = t60029 * t1385;
    let t77072 = t6284 * t1610;
    let t77753 = t2104 * t167;
    (t60988, t61287, t62417, t62923, t63256, t75638, t77072, t77753)
}
