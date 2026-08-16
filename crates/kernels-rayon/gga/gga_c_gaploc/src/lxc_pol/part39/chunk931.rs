//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 931/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk931(t40301: f64, t41809: f64, t6508: f64, t4820: f64, t6824: f64, t10418: f64, t2389: f64, t34506: f64, t34507: f64, t41726: f64, t12766: f64, t1572: f64, t4673: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41992 = 0.38342925953920749676e1_f64 * t40301;
    let t41993 = t6508 * t41809;
    let t41996 = 0.79445533226334281487e-1_f64 * t6824 * t4820 * t41993;
    let t42001 = t10418 * t2389;
    let t42005 = 0.85801175884441024004e1_f64 * t34506 * t34507 * t41726;
    let t42008 = 0.47667319935800568892e0_f64 * t1572 * t4673 * t12766;
    (t41992, t41993, t41996, t42001, t42005, t42008)
}
