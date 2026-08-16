//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 949/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk949<F: Float>(t2977: F, t484: F, t5042: F, t691: F, t276: F, t40: F, t4027: F, t1284: F, t228: F, t1292: F, t3937: F, t5351: F) -> (F, F, F, F, F, F) {
    let t15018 = t2977 * t484;
    let t15043 = t5042 * t691;
    let t15050 = t40 * t4027 * t276;
    let t15072 = F::cast_from(16.0_f64) * t1284 * t228;
    let t15095 = F::cast_from(16.0_f64) * t1292 * t228;
    let t15106 = t3937 * t5351;
    (t15018, t15043, t15050, t15072, t15095, t15106)
}
