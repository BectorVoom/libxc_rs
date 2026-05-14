//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 738/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk738<F: Float>(t12881: F, t9497: F, t8248: F, t9565: F, t40301: F, t41809: F, t6508: F, t4820: F, t6824: F, t34506: F, t34507: F, t41726: F, t12766: F, t1572: F, t4673: F, t12919: F, t4953: F) -> (F, F, F, F, F, F, F, F) {
    let t41989 = 0.25025342966295298669e1 * t9497 * t12881;
    let t41991 = 0.11916829983950142223e0 * t8248 * t9565;
    let t41992 = 0.38342925953920749676e1 * t40301;
    let t41993 = t6508 * t41809;
    let t41996 = 0.79445533226334281487e-1 * t6824 * t4820 * t41993;
    let t42005 = 0.85801175884441024004e1 * t34506 * t34507 * t41726;
    let t42008 = 0.47667319935800568892e0 * t1572 * t4673 * t12766;
    let t42018 = 0.69017266717057349418e1 * t4953 * t12919;
    (t41989, t41991, t41992, t41993, t41996, t42005, t42008, t42018)
}
