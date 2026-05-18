//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1323/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1323<F: Float>(t11906: F, t11909: F, t11914: F, t11921: F, t1670: F, t1674: F, t20034: F, t20036: F, t20037: F, t20038: F, t20039: F, t3984: F, t4099: F, t5645: F, t6596: F, t694: F, t695: F) -> F {
    let t24643 = F::new(6.0) * t1670 * t4099 * t694 + F::new(24.0) * t1674 * t5645 * t695 + F::new(12.0) * t3984 * t6596 * t694 + t11906 - t11909 - t11914 - t11921 + t20034 + t20036 + t20037 + t20038 - t20039;
    t24643
}
