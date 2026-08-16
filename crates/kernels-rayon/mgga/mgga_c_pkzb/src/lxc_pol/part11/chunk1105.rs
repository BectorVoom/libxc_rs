//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1105/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1105(t1088: f64, t5829: f64, t1107: f64, t1977: f64, t1954: f64, t2826: f64, t1100: f64, t5498: f64, t1976: f64, t20716: f64, t1937: f64, t2793: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21090 = t1088 * t5829;
    let t21093 = t1977 * t1107;
    let t21143 = t2826 * t1954;
    let t21146 = t1100 * t5498;
    let t21156 = t2826 * t1976;
    let t21165 = 0.68493333333333333332e-1_f64 * t20716;
    let t21179 = t2793 * t1937;
    (t21090, t21093, t21143, t21146, t21156, t21165, t21179)
}
