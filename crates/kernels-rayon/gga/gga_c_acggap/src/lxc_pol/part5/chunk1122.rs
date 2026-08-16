//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1122/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1122(t12673: f64, t11954: f64, t12126: f64, t12130: f64, t12148: f64, t12156: f64, t12661: f64, t12664: f64, t12669: f64, t12672: f64, t12677: f64, t20043: f64, t20046: f64, t20048: f64, t20049: f64, t20052: f64, t20053: f64, t20054: f64) -> (f64, f64) {
    let t20055 = 0.96319466275353142156e0_f64 * t12673;
    let t20056 = -t20043 - t20046 - t20048 - t20049 - t11954 + t20052 + t12148 + t12156 - t20053 - t12661 - t12664 - t20054 - t12669 + t12672 + t20055 + t12677 - t12126 + t12130;
    (t20055, t20056)
}
