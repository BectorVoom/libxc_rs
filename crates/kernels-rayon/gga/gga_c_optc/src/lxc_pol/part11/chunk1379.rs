//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1379/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1379(t1512: f64, t424: f64, t23518: f64, t487: f64, t5239: f64, t17643: f64, t4305: f64, t15066: f64, t15067: f64, t5096: f64, t43636: f64, t5101: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t58547 = 1.0_f64 / t424 / t1512;
    let t58560 = t23518 * t487;
    let t58563 = t5239 * t5239;
    let t58572 = 0.2077890707925103596e3_f64 * t4305 * t17643;
    let t58581 = t15066 * t15067 * t5096;
    let t58585 = t43636 * t15067 * t5101;
    (t58547, t58560, t58563, t58572, t58581, t58585)
}
