//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1143/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1143<F: Float>(t37078: F, t40782: F, t40798: F, t40805: F, t40807: F, t41858: F, t41864: F, t42491: F, t42493: F, t42495: F, t42497: F, t42500: F, t42502: F, t42505: F, t42508: F, t42512: F, t42516: F, t42519: F) -> F {
    let t42521 = t42491 / F::cast_from(2.0_f64) + t42493 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t42495 + t42497 / F::cast_from(4.0_f64) + t42500 / F::cast_from(4.0_f64) - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t42502 + F::cast_from(2.0_f64) * t42505 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t42508 - t41858 + t40782 + F::cast_from(22.0_f64) / F::cast_from(9.0_f64) * t37078 + t41864 - t40798 - t40805 - t40807 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t42512 + F::cast_from(3.0_f64) * t42516 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t42519;
    t42521
}
