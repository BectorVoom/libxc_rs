//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 704/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk704(t3040: f64, t3267: f64, t10012: f64, t1022: f64, t9438: f64, t2684: f64, t10007: f64, t825: f64, t313: f64, t9014: f64, t1645: f64, t3251: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13140 = 0.35750489951850426669e0_f64 * t3267 * t3040;
    let t13141 = t10012 * t1022;
    let t13142 = t9438 * t13141;
    let t13143 = t2684 * t13142;
    let t13144 = 0.15976219147466979032e-1_f64 * t13143;
    let t13149 = t10007 * t1022;
    let t13150 = t9438 * t13149;
    let t13151 = t825 * t13150;
    let t13152 = 0.15976219147466979032e-1_f64 * t13151;
    let t13153 = t313 * t9014;
    let t13154 = t1645 * t3251;
    (t13140, t13141, t13142, t13144, t13149, t13150, t13152, t13153, t13154)
}
