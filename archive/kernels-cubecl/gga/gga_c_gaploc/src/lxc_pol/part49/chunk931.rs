//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 931/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk931<F: Float>(t40301: F, t41809: F, t6508: F, t4820: F, t6824: F, t10418: F, t2389: F, t34506: F, t34507: F, t41726: F, t12766: F, t1572: F, t4673: F) -> (F, F, F, F, F, F) {
    let t41992 = F::cast_from(0.38342925953920749676e1_f64) * t40301;
    let t41993 = t6508 * t41809;
    let t41996 = F::cast_from(0.79445533226334281487e-1_f64) * t6824 * t4820 * t41993;
    let t42001 = t10418 * t2389;
    let t42005 = F::cast_from(0.85801175884441024004e1_f64) * t34506 * t34507 * t41726;
    let t42008 = F::cast_from(0.47667319935800568892e0_f64) * t1572 * t4673 * t12766;
    (t41992, t41993, t41996, t42001, t42005, t42008)
}
