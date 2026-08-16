//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1270/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1270(t1364: f64, t2436: f64, t4715: f64, t782: f64, t4758: f64, t8279: f64, t4630: f64, t645: f64, t1232: f64, t17785: f64, t1268: f64, t5366: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44169 = t2436 * t1364;
    let t44960 = t4715 * t782;
    let t44994 = t4758 * t782;
    let t45241 = t4715 * t8279;
    let t50656 = t4630 * t645;
    let t51545 = t17785 * t1232;
    let t51622 = t5366 * t1268;
    (t44169, t44960, t44994, t45241, t50656, t51545, t51622)
}
