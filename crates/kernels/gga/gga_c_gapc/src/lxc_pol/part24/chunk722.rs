//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 722/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk722<F: Float>(t8570: F, t8571: F, t1595: F, t2890: F, t473: F, t1037: F, t1416: F, t4687: F, t2936: F, t2948: F, t518: F, t1460: F, t2954: F) -> (F, F, F, F, F) {
    let t8572 = t8570 * t8571;
    let t8574 = t2890 * t1595;
    let t8575 = t473 * t8574;
    let t8577 = t1416 * t1037;
    let t8578 = t8577 * t4687;
    let t8579 = t2936 * t8578;
    let t8581 = t518 * t2948;
    let t8583 = t1460 * t2954;
    (t8572, t8575, t8579, t8581, t8583)
}
