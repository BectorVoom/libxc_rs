//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1484/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1484<F: Float>(t31032: F, t31284: F, t116912: F, t31261: F, t10208: F, t69: F, t96: F, t100: F, t1513: F, t2339: F, t31027: F, t31268: F) -> (F, F, F, F, F, F) {
    let t117484 = F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t31032 * t31284;
    let t117497 = F::cast_from(4.0_f64) * t116912 * t31261;
    let t117499 = t69 * t10208 * t96;
    let t117500 = t100 * t1513;
    let t117505 = t69 * t2339 * t96;
    let t117510 = F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t31027 * t31268;
    (t117484, t117497, t117499, t117500, t117505, t117510)
}
