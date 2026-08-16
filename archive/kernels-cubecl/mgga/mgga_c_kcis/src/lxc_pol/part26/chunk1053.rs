//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1053/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1053<F: Float>(t2153: F, t9268: F, t7669: F, t906: F, t209: F, t2403: F, t2404: F, t706: F, t7589: F, t2387: F, t73: F, t9251: F) -> (F, F, F, F, F) {
    let t26558 = t9268 * t2153;
    let t26561 = t7669 * t906;
    let t26571 = t209 * t2403 * t706 * t2404;
    let t26572 = t7589 * t26571;
    let t26576 = t209 * t73 * t9251 * t2387;
    (t26558, t26561, t26571, t26572, t26576)
}
