//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1016/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1016<F: Float>(t3409: F, t4443: F, t1511: F, t3573: F, t1089: F, t175: F, t322: F, t384: F, t4099: F, t1008: F, t4764: F, t1095: F, t398: F, t4838: F) -> (F, F, F, F, F) {
    let t17205 = t3409 * t4443;
    let t17216 = t3573 * t1511;
    let t17221 = t384 * t1089 * t175 * t4099 * t322;
    let t17223 = t1008 * t4764;
    let t17228 = t384 * t398 * t1095 * t4838 * t322;
    (t17205, t17216, t17221, t17223, t17228)
}
