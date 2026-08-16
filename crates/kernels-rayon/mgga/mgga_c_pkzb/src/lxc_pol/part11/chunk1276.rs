//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1276/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1276(t11536: f64, t2464: f64, t11365: f64, t2328: f64, t3162: f64, t9762: f64, t12919: f64, t1306: f64, t26901: f64, t31052: f64, t31055: f64, t31057: f64, t31061: f64, t31092: f64, t31094: f64, t31096: f64, t955: f64) -> (f64, f64, f64) {
    let t31097 = t11536 * t2464;
    let t31104 = 0.35089341735807877242e1_f64 * t2328 * t11365;
    let t31106 = 0.51947577317044391276e2_f64 * t9762 * t3162;
    let t31107 = 6.0_f64 * t12919 * t1306 * t26901 - t1306 * t31097 * t955 + t31052 - t31055 - t31057 - t31061 - t31092 - t31094 + t31096 - t31104 - t31106;
    (t31104, t31106, t31107)
}
