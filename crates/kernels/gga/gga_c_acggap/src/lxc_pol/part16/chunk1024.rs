//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1024/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1024<F: Float>(t36039: F, t31142: F, t8810: F, t7440: F, t8803: F, t1181: F, t4623: F, t604: F, t7426: F, t30090: F, t8897: F, t31362: F, t8903: F) -> (F, F, F, F, F, F) {
    let t36040 = F::new(7.0) / F::new(24.0) * t36039;
    let t36041 = t31142 * t8810;
    let t36042 = F::new(7.0) / F::new(72.0) * t36041;
    let t36065 = t7440 * t8803;
    let t36066 = F::new(11.0) / F::new(288.0) * t36065;
    let t36081 = t7426 * t1181 * t604 * t4623;
    let t36082 = F::new(0.62896184579208304136e-3) * t36081;
    let t36083 = t30090 * t8897;
    let t36085 = t31362 * t8903;
    (t36040, t36042, t36066, t36082, t36083, t36085)
}
