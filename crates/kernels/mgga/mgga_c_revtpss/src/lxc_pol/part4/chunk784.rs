//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 784/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk784<F: Float>(t3: F, t4153: F, t116: F, t2327: F, t117: F, t2371: F, t1459: F, t1461: F, t572: F, t573: F, t2219: F, t2223: F, t2226: F, t2230: F, t2233: F, t2239: F) -> (F, F, F, F, F, F) {
    let t4154 = t3 * t4153;
    let t4158 = param_d * t4153;
    let t4162 = t116 * t2327;
    let t4165 = t117 * t2371;
    let t4168 = F::new(6.0) * t1459 * t1461 + t4158 * t573 + F::new(6.0) * t4162 * t572 + F::new(3.0) * t4165 * t572;
    let t4171 = -t2219 + t2223 - t2226 + t2230 - t2233 + t2239;
    (t4154, t4158, t4162, t4165, t4168, t4171)
}
