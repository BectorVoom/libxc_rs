//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3654/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3654<F: Float>(t56176: F, t56183: F, t56185: F, t56187: F, t56189: F, t56209: F, t56212: F, t56214: F, t56216: F, t56228: F, t68363: F, t68366: F) -> F {
    let t69072 = -F::cast_from(0.28493333333333333333e0_f64) * t68363 + F::cast_from(0.79148148148148148147e-1_f64) * t68366 - F::cast_from(0.21106172839506172839e-1_f64) * t56176 + F::cast_from(0.63318518518518518517e-1_f64) * t56183 - F::cast_from(0.47488888888888888888e-1_f64) * t56185 - F::cast_from(0.23744444444444444444e-1_f64) * t56187 - F::cast_from(0.71233333333333333332e-1_f64) * t56189 + F::cast_from(0.15829629629629629629e-1_f64) * t56209 + F::cast_from(0.79148148148148148147e-2_f64) * t56212 + F::cast_from(0.47488888888888888888e-1_f64) * t56214 - F::cast_from(0.13191358024691358025e-1_f64) * t56216 + F::cast_from(0.31659259259259259258e-1_f64) * t56228;
    t69072
}
