//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 727/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk727(t12: f64, t1835: f64, t87: f64, t5519: f64, t210: f64, t173: f64, t4932: f64) -> (f64, f64, f64, f64) {
    let t5528 = 1.0_f64 / t87 / t1835 / t12;
    let t5543 = 0.93932222222222222223e0_f64 * t5519;
    let t5547 = 1.0_f64/pow_3_2(t210);
    let t5555 = t4932 * t173;
    (t5528, t5543, t5547, t5555)
}
