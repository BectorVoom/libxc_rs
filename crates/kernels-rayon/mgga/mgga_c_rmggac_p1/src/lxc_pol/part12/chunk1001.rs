//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1001/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1001(t27091: f64, t40901: f64, t40487: f64, t5148: f64, t39059: f64, t5271: f64, t39063: f64, t5259: f64, t2402: f64, t839: f64, t2367: f64, t321: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41077 = t27091 * t40901;
    let t41079 = t5148 * t40487;
    let t41084 = t5271 * t39059;
    let t41086 = t5259 * t39063;
    let t41088 = t2402 * t839;
    let t41091 = t2367 * t321;
    (t41077, t41079, t41084, t41086, t41088, t41091)
}
