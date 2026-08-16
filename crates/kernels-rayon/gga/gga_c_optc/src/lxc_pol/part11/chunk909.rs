//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 909/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk909(t17022: f64, t17078: f64, t1411: f64, t4990: f64, t3861: f64, t5049: f64, t17064: f64, t914: f64, t17060: f64, t3813: f64, t4961: f64, t3885: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17079 = t17022 + t17078;
    let t17092 = t4990 * t1411;
    let t17096 = t3861 * t5049;
    let t17106 = t914 * t17064;
    let t17109 = t914 * t17060;
    let t17114 = t3813 * t4961;
    let t17115 = t3885 * t17114;
    (t17079, t17092, t17096, t17106, t17109, t17114, t17115)
}
