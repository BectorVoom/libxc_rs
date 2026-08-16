//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1240/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1240<F: Float>(t40779: F, t40781: F, t40788: F, t40797: F, t40804: F, t40806: F, t40808: F, t37043: F, t37048: F, t37055: F, t37069: F, t37078: F, t40790: F, t40792: F, t40794: F, t40800: F, t40802: F, t40812: F) -> F {
    let t41858 = F::cast_from(22.0_f64) / F::cast_from(9.0_f64) * t40779;
    let t41859 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t40781;
    let t41864 = F::cast_from(44.0_f64) / F::cast_from(9.0_f64) * t40788;
    let t41867 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t40797;
    let t41870 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t40804;
    let t41871 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t40806;
    let t41872 = F::cast_from(22.0_f64) / F::cast_from(9.0_f64) * t40808;
    let t41875 = -t41858 + t41859 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t37048 + F::cast_from(4.0_f64) * t37055 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t37069 + F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t37078 + t41864 + t40790 / F::cast_from(2.0_f64) + t40792 / F::cast_from(2.0_f64) + t40794 - t41867 + t40800 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t40802 - t41870 - t41871 + t41872 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t37043 - F::cast_from(3.0_f64) * t40812;
    t41875
}
