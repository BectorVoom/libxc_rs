//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1141/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1141<F: Float>(t40260: F, t3718: F, t5086: F, t11002: F, t1115: F, t2847: F, t40781: F, t40797: F, t40804: F, t40806: F, t40821: F, t40840: F) -> (F, F, F, F, F, F, F, F, F) {
    let t41776 = F::cast_from(0.27944763721877274748e0_f64) * t40260;
    let t41791 = t5086 * t3718;
    let t41816 = t11002 * t1115 * t2847;
    let t41859 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t40781;
    let t41867 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t40797;
    let t41870 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t40804;
    let t41871 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t40806;
    let t41877 = F::cast_from(8.0_f64) * t40821;
    let t41885 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t40840;
    (t41776, t41791, t41816, t41859, t41867, t41870, t41871, t41877, t41885)
}
