//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1113/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1113(t11230: f64, t1282: f64, t15692: f64, t1872: f64, t27141: f64, t28265: f64, t29029: f64, t29031: f64, t29041: f64, t29082: f64, t29186: f64, t29188: f64, t29214: f64, t437: f64, t6860: f64, t6879: f64, t7809: f64, t8108: f64) -> f64 {
    let t29216 = -6.0_f64 * t11230 * t29188 - t1282 * t29214 + 4.0_f64 * t15692 * t8108 - 2.0_f64 * t1872 * t28265 + 2.0_f64 * t27141 * t6860 + t29186 * t437 - t6879 * t7809 - t29029 + t29031 - t29041 + t29082;
    t29216
}
