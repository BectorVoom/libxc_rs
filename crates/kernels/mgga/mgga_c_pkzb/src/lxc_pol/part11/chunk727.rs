//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 727/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk727<F: Float>(t12: F, t1835: F, t87: F, t5519: F, t210: F, t173: F, t4932: F) -> (F, F, F, F) {
    let t5528 = F::cast_from(1.0_f64) / t87 / t1835 / t12;
    let t5543 = F::cast_from(0.93932222222222222223e0_f64) * t5519;
    let t5547 = F::cast_from(1.0_f64)/pow_3_2::<F>(t210);
    let t5555 = t4932 * t173;
    (t5528, t5543, t5547, t5555)
}
