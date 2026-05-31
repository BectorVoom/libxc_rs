//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 338/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk338<F: Float>(t1201: F, t378: F, t1169: F, t1175: F, t1178: F, t1182: F, t884: F, t887: F) -> (F, F) {
    let t1202 = t1201 * t378;
    let t1208 = F::cast_from(0.258925e1_f64) * t1175 - t884 + F::cast_from(0.905775e0_f64) * t1169 + F::cast_from(0.16504875e0_f64) * t1178 - t887 + F::cast_from(0.248355e0_f64) * t1182;
    (t1202, t1208)
}
