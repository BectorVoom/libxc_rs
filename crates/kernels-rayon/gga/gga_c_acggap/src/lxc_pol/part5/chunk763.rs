//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 763/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk763(t129: f64, t145: f64, t5: f64, t5784: f64, t1755: f64, t3382: f64, t1754: f64, t4680: f64, t1753: f64, t4289: f64, t1181: f64, t1487: f64, t157: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5787 = t129 * t5 * t5784 * t145;
    let t5790 = t3382 * t1755;
    let t5792 = t4680 * t1754;
    let t5795 = t4289 * t1753;
    let t5796 = t1181 * t5795;
    let t5799 = t157 * t1487;
    (t5787, t5790, t5792, t5795, t5796, t5799)
}
