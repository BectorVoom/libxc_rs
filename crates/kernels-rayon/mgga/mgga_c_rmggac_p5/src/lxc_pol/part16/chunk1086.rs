//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1086/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1086(t5840: f64, t699: f64, t35204: f64, t35208: f64, t35226: f64, t35230: f64, t35242: f64, t35246: f64, t37375: f64, t45696: f64, t45701: f64, t45709: f64, t45716: f64, t45722: f64, t45724: f64, t45728: f64, t45732: f64, t45734: f64, t739: f64) -> (f64, f64) {
    let t48591 = t699 * t5840;
    let t48609 = -0.59871208509319042821e-1_f64 * t739 * t48591 - 0.425645998932978394e-4_f64 * t45696 + 0.638468998399467591e-4_f64 * t45701 - 0.38422568777328955681e-2_f64 * t35204 + 0.92232789896410962673e-3_f64 * t35208 + 0.60975299583150056624e-3_f64 * t35226 - 0.86737941314158990616e-4_f64 * t35230 + t37375 + 0.72042316457491791901e-3_f64 * t45709 - 0.10248087766267884741e-3_f64 * t45716 + 0.60975299583150056624e-3_f64 * t35242 - 0.86737941314158990616e-4_f64 * t35246 + 0.81823984962736025192e-1_f64 * t45722 - 0.16364796992547205038e0_f64 * t45724 - 0.16364796992547205038e0_f64 * t45728 - 0.16364796992547205038e0_f64 * t45732 + 0.5107751987195740728e-4_f64 * t45734;
    (t48591, t48609)
}
