//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 923/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk923<F: Float>(t14106: F, t425: F, t431: F, t438: F, t1195: F, t3228: F, t1200: F, t1205: F, t3770: F, t993: F, t1032: F, t3697: F) -> (F, F, F, F, F, F, F, F) {
    let t14107 = t14106 * t425;
    let t14109 = t14106 * t431;
    let t14111 = t14106 * t438;
    let t14113 = t3228 * t1195;
    let t14115 = t3228 * t1200;
    let t14117 = t3228 * t1205;
    let t14120 = F::cast_from(0.12004725073059526352e-1_f64) * t3770 * t993;
    let t14122 = F::cast_from(0.40015750243531754508e-2_f64) * t1032 * t3697;
    (t14107, t14109, t14111, t14113, t14115, t14117, t14120, t14122)
}
