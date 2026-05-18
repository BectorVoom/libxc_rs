//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1143/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1143<F: Float>(t37078: F, t40782: F, t40798: F, t40805: F, t40807: F, t41858: F, t41864: F, t42491: F, t42493: F, t42495: F, t42497: F, t42500: F, t42502: F, t42505: F, t42508: F, t42512: F, t42516: F, t42519: F) -> F {
    let t42521 = t42491 / F::new(2.0) + t42493 / F::new(2.0) - F::new(3.0) / F::new(4.0) * t42495 + t42497 / F::new(4.0) + t42500 / F::new(4.0) - F::new(4.0) / F::new(3.0) * t42502 + F::new(2.0) * t42505 - F::new(2.0) / F::new(3.0) * t42508 - t41858 + t40782 + F::new(22.0) / F::new(9.0) * t37078 + t41864 - t40798 - t40805 - t40807 - F::new(3.0) / F::new(2.0) * t42512 + F::new(3.0) * t42516 - F::new(3.0) / F::new(2.0) * t42519;
    t42521
}
