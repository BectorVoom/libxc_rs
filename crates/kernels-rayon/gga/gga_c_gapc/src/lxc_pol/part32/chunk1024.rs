//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1024/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1024(t4054: f64, t6: f64, t4687: f64, t5407: f64, t505: f64, t681: f64, t5199: f64, t5214: f64, t5217: f64, t5216: f64, t5215: f64, t1509: f64, t1666: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21283 = t4054 * t6;
    let t21369 = t5407 * t4687;
    let t21625 = t681 * t505;
    let t21631 = t5214 * t5199 * t5217;
    let t21642 = t5216 * t6;
    let t21643 = t5215 * t21642;
    let t21649 = t1666 * t1509;
    (t21283, t21369, t21625, t21631, t21642, t21643, t21649)
}
