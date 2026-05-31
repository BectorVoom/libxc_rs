//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 363/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk363<F: Float>(t505: F, t6135: F, t2354: F, t446: F, t6008: F, t713: F, t193: F, t89: F, t6061: F, t676: F, t27: F, t6113: F, t6117: F, t6122: F, t6126: F, t6130: F, t6134: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6136 = t6135 * t505;
    let t6137 = t2354 * t6136;
    let t6138 = t446 * t6137;
    let t6140 = t6008 * t713;
    let t6141 = t193 * t6140;
    let t6142 = t89 * t6141;
    let t6144 = t676 * t6061;
    let t6146 = t89 * t27 * t6144;
    let t6148 = t6113 / F::cast_from(12.0_f64) + t6117 + t6122 / F::cast_from(18.0_f64) + t6126 / F::cast_from(3.0_f64) - t6130 / F::cast_from(6.0_f64) + t6134 + t6138 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6142 - t6146 / F::cast_from(3.0_f64);
    (t6136, t6137, t6138, t6140, t6141, t6142, t6144, t6146, t6148)
}
