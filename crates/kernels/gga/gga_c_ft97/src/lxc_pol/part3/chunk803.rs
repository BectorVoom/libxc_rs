//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 803/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk803<F: Float>(t16345: F, t16357: F, t16365: F, t16478: F, t488: F, t83: F, t379: F, t4572: F, t8557: F, t1882: F, t4553: F, t15604: F, t15606: F, t15609: F, t15612: F, t15617: F, t15621: F, t15628: F, t15888: F, t15891: F, t15894: F, t15897: F) -> (F, F, F, F, F) {
    let t16480 = t16345 + t16357 + t16365 + t16478;
    let t16481 = t488 * t16480;
    let t16482 = t83 * t16481;
    let t16485 = t4572 * t379;
    let t16486 = t8557 * t16485;
    let t16490 = t1882 * t4553;
    let t16503 = -F::new(2.0) * t15604 + F::new(2.0) / F::new(81.0) * t15606 - F::new(2.0) / F::new(27.0) * t15609 + t15612 / F::new(27.0) + F::new(2.0) / F::new(3.0) * t15617 + F::new(4.0) / F::new(3.0) * t15621 - t15628 / F::new(9.0) - t15888 / F::new(3.0) + t15891 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t15894 - F::new(8.0) / F::new(27.0) * t15897;
    (t16481, t16482, t16486, t16490, t16503)
}
