//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 303/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk303<F: Float>(t1165: F, t1167: F, t1169: F, t1152: F, t1154: F, t1158: F, t1161: F, t14: F, t351: F, t394: F, t31: F, t4: F, t96: F) -> (F, F) {
    let t1171 = -F::cast_from(0.44044444444444444445e-2_f64) * t1165 + F::cast_from(0.88088888888888888889e-2_f64) * t1167 + F::cast_from(0.55033333333333333333e-2_f64) * t1169;
    let t1174 = -t1152 * t1154 / F::new(18.0) - t1158 * t351 / F::new(6.0) + t394 * t1161 / F::new(9.0) + t14 * t1171 / F::new(2.0);
    let t1179 = F::cast_from(0.14764770444444444444e-2_f64) * t4 * t96 * t31;
    (t1174, t1179)
}
