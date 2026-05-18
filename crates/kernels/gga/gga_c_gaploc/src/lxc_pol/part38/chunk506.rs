//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 506/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk506<F: Float>(t4391: F, t9558: F, t544: F, t6851: F, t2326: F, t900: F, t1407: F, t3178: F, t3163: F, t4379: F, t2293: F, t2366: F) -> (F, F, F, F, F, F) {
    let t9560 = F::new(0.59584149919750711116e-1) * t4391 * t9558;
    let t9561 = t544 * t6851;
    let t9562 = t900 * t2326;
    let t9564 = F::new(0.89376224879626066674e-1) * t9561 * t9562;
    let t9568 = t1407 * t3178;
    let t9569 = F::new(0.38342925953920749676e0) * t9568;
    let t9571 = F::new(0.29792074959875355558e-1) * t4379 * t3163;
    let t9572 = t2366 * t2293;
    (t9560, t9562, t9564, t9569, t9571, t9572)
}
