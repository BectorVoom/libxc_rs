//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1372/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1372(t26149: f64, t8607: f64, t26161: f64, t33221: f64, t92200: f64, t1388: f64, t92169: f64, t120016: f64, t1983: f64, t2095: f64, t31669: f64, t5161: f64) -> (f64, f64, f64, f64, f64) {
    let t121181 = t8607 * t26149;
    let t121184 = 2.0_f64 * t26161 * t92200 * t33221;
    let t121190 = 6.0_f64 * t26161 * t92169 * t33221 * t1388;
    let t121192 = t1983 * t2095 * t120016;
    let t121194 = t1983 * t31669 * t5161;
    (t121181, t121184, t121190, t121192, t121194)
}
