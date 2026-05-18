//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1253/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1253<F: Float>(t10924: F, t5679: F, t6096: F, t11069: F, t5669: F, t20671: F, t25070: F, t28856: F, t11029: F, t2087: F, t4614: F, t10951: F, t5782: F) -> (F, F, F, F, F) {
    let t33269 = F::new(0.71500979903700853338e0) * t5679 * t10924 * t6096;
    let t33271 = F::new(0.2044956050875773316e1) * t5669 * t11069;
    let t33273 = t28856 * t20671 * t25070;
    let t33274 = F::new(0.2556195063594716645e0) * t33273;
    let t33282 = F::new(0.18404604457881959845e2) * t2087 * t4614 * t11029;
    let t33284 = F::new(0.18404604457881959845e2) * t5782 * t10951;
    (t33269, t33271, t33274, t33282, t33284)
}
