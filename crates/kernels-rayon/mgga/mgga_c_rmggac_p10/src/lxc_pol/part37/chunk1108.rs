//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1108/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1108(t118: f64, t321: f64, t5259: f64, t72011: f64, t76292: f64, t76311: f64, t76319: f64, t76322: f64, t78028: f64, t78031: f64, t78034: f64, t78036: f64, t78038: f64, t78039: f64, t78040: f64, t80192: f64, t80452: f64) -> f64 {
    let t80472 = t76292 - 0.39914139006212695214e-1_f64 * t118 * t80192 - t78028 + t72011 + t78031 + t78034 + t76311 - t78036 - t78038 + t78039 + t78040 + 0.11974241701863808564e0_f64 * t5259 * t80452 * t321 + t76319 + t76322;
    t80472
}
