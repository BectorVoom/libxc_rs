//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1408/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1408(t12038: f64, t12065: f64, t12116: f64, t12117: f64, t12128: f64, t1625: f64, t1641: f64, t193: f64, t35123: f64, t35126: f64, t35128: f64, t35130: f64, t35133: f64, t35136: f64, t35138: f64, t35140: f64, t35142: f64, t35144: f64, t35146: f64, t4379: f64, t524: f64, t541: f64) -> f64 {
    let t38836 = -t35123 + t35126 + 0.79445533226334281486e-1_f64 * t4379 * t12038 + t35128 - t35130 - t35133 + t35136 - t35138 + t35140 + t35142 + t35144 - t35146 - 0.61348681526273199482e1_f64 * t1641 * t12128 + 0.71500979903700853338e0_f64 * t524 * t12116 * t193 + 0.47667319935800568892e0_f64 * t12117 * t541 + 0.35750489951850426669e0_f64 * t1625 * t12065;
    t38836
}
