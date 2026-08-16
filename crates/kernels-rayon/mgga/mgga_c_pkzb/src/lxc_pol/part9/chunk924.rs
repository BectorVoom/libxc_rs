//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 924/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk924(t5179: f64, t4996: f64, t5005: f64, t5011: f64, t5019: f64, t5022: f64, t5154: f64, t5170: f64, t7030: f64, t7031: f64, t7032: f64, t7034: f64, t7037: f64, t7039: f64, t7041: f64, t7042: f64) -> (f64, f64) {
    let t7043 = 24.0_f64 * t5179;
    let t7044 = t7030 - t5154 - t7031 - t7032 + t4996 + t5005 - t5011 + t5170 - t7034 - t7037 - t7039 + t7041 + t5019 - t5022 - t7042 - t7043;
    (t7043, t7044)
}
