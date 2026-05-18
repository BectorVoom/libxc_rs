//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1051/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1051<F: Float>(t13277: F, t1646: F, t3203: F, t3316: F, t3202: F, t3200: F, t1800: F, t2829: F, t2845: F, t4554: F, t1804: F, t3210: F) -> (F, F, F, F, F) {
    let t13278 = F::new(0.33163888888888888888e-2) * t13277;
    let t13280 = t3203 * t1646 * t3316;
    let t13281 = t3202 * t13280;
    let t13282 = t3200 * t13281;
    let t13284 = t1800 * t2829;
    let t13285 = t3202 * t13284;
    let t13286 = t3200 * t13285;
    let t13288 = t1800 * t2845;
    let t13289 = t3202 * t13288;
    let t13290 = t4554 * t13289;
    let t13292 = t1804 * t2829;
    let t13293 = t3210 * t13292;
    (t13278, t13282, t13286, t13290, t13293)
}
