//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 629/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk629<F: Float>(t6032: F, t1711: F, t229: F, t2806: F, t2812: F, t2989: F, t2995: F, t5038: F, t5044: F, t6020: F, t6023: F, t6026: F, t6027: F, t6028: F, t6029: F, t6030: F, t6031: F) -> F {
    let t6033 = F::cast_from(4.0_f64) * t6032;
    let t6034 = t229 * t1711;
    let t6035 = F::cast_from(4.0_f64) * t6034;
    let t6036 = -t6020 - t2989 + t2806 - t2812 - t6023 - t6026 - t5038 + t2995 + t6027 + t6028 - t6029 - t6030 - t5044 - t6031 + t6033 - t6035;
    t6036
}
