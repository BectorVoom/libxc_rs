//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 971/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk971<F: Float>(t10649: F, t10797: F, t14718: F, t14892: F, t14899: F, t14949: F, t14951: F, t15058: F, t15062: F, t15065: F, t15069: F, t14927: F, t14939: F, t14947: F) -> F {
    let t15071 = -F::new(22.0) / F::new(9.0) * t14718 - t10649 - t14892 - t14949 + F::new(2.0) / F::new(3.0) * t14899 + t14951 + t15058 / F::new(2.0) - t10797 - t15062 / F::new(2.0) - t15065 / F::new(4.0) + F::new(3.0) / F::new(8.0) * t15069;
    let t15073 = t14927 + t14939 + t14947 + t15071;
    t15073
}
