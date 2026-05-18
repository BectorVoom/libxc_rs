//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 626/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk626<F: Float>(t14715: F, t14895: F, t10580: F, t2: F, t1232: F, t1771: F, t1228: F, t8282: F, t2347: F, t852: F, t2360: F, t1212: F, t2781: F) -> (F, F, F, F, F, F, F, F) {
    let t14946 = F::new(4.0) / F::new(27.0) * t14715;
    let t14949 = F::new(4.0) / F::new(9.0) * t14895;
    let t14961 = t10580 * t2;
    let t15011 = t1771 * t1232;
    let t15025 = t8282 * t1228;
    let t15042 = t852 * t2347;
    let t15047 = t852 * t2360;
    let t15051 = t2781 * t1212;
    (t14946, t14949, t14961, t15011, t15025, t15042, t15047, t15051)
}
