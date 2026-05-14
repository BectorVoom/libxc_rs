//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 944/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk944<F: Float>(t15609: F, t8139: F, t8140: F, t941: F, t186: F, t2786: F, t2579: F, t923: F, t6: F, t11925: F, t286: F, t2206: F, t869: F, t2674: F, t1087: F, t2188: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15610 = t8139 * t15609;
    let t15615 = t941 * t8140;
    let t15644 = t2786 * t186;
    let t15650 = t2579 * t923;
    let t15679 = t8140 * t6;
    let t15680 = t8139 * t15679;
    let t15699 = t11925 * t286;
    let t15805 = t869 * t2206;
    let t15811 = t2674 * t186;
    let t15835 = t1087 * t2188;
    (t15610, t15615, t15644, t15650, t15679, t15680, t15699, t15805, t15811, t15835)
}
