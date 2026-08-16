//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 477/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk477(t383: f64, t5914: f64, t1058: f64, t1610: f64, t1630: f64, t1632: f64, t3186: f64, t3200: f64, t353: f64, t384: f64, t4669: f64, t5903: f64, t5929: f64, t5933: f64, t5937: f64, t5939: f64) -> f64 {
    let t5941 = t383 * t5914;
    let t5943 = 2.0_f64 * t1058 * t5933 + t1058 * t5937 + 2.0_f64 * t1610 * t1632 + 2.0_f64 * t1630 * t4669 + 2.0_f64 * t3186 * t5929 - t3200 * t5939 + t353 * t5941 + t384 * t5903;
    t5943
}
