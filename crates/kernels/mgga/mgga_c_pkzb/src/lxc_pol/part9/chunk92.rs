//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 92/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk92<F: Float>(t218: F, t219: F, t220: F, t210: F, t213: F, t216: F) -> (F, F, F, F) {
    let t222 = t218 * t219 * t220;
    let t224 = F::cast_from(0.379785e1_f64) * t213 + F::cast_from(0.8969e0_f64) * t210 + F::cast_from(0.204775e0_f64) * t216 + F::cast_from(0.123235e0_f64) * t222;
    let t227 = F::cast_from(1.0_f64) + F::cast_from(0.16081979498692535067e2_f64) / t224;
    let t228 = F::ln(t227);
    (t222, t224, t227, t228)
}
