//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1254/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1254<F: Float>(t40779: F, t40786: F, t40788: F, t41859: F, t41867: F, t41870: F, t41871: F, t42491: F, t42493: F, t42495: F, t42497: F, t42500: F, t42502: F, t42505: F, t42508: F, t42512: F, t42516: F, t42519: F) -> F {
    let t44630 = t42491 + t42493 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t42495 + t42497 / F::cast_from(2.0_f64) + t42500 / F::cast_from(2.0_f64) - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t42502 + F::cast_from(4.0_f64) * t42505 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t42508 - F::cast_from(44.0_f64) / F::cast_from(9.0_f64) * t40779 + t41859 + t40786 + F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t40788 - t41867 - t41870 - t41871 - F::cast_from(3.0_f64) * t42512 + F::cast_from(6.0_f64) * t42516 - F::cast_from(3.0_f64) * t42519;
    t44630
}
