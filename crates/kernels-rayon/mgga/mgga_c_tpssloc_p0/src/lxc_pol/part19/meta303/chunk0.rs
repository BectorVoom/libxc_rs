//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1090/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1090(t3014: f64, t343: f64, t12461: f64, t3698: f64, t3475: f64, t460: f64, t20: f64, t60: f64, t9108: f64, t94: f64, t102: f64, t9174: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23547 = t3014 * t343;
    let t23857 = t12461 * t3698;
    let t24705 = t3475 * t460;
    let t32253 = 1.0_f64 / t60 / t20;
    let t35577 = t94 * t9108;
    let t35761 = t102 * t9174;
    (t23547, t23857, t24705, t32253, t35577, t35761)
}
