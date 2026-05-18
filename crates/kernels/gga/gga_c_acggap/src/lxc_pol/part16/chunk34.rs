//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 34/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk34<F: Float>(t43: F, t50: F, t45: F, t47: F, t52: F, zeta_threshold: F) -> (F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t97 = t45 * t45;
    let t98 = t47 * t47;
    let t99 = piecewise3::<f64>(t44, t97, t98);
    let t100 = t52 * t52;
    let t101 = piecewise3::<f64>(t51, t97, t100);
    let t103 = t99 / F::new(2.0) + t101 / F::new(2.0);
    (t98, t100, t103)
}
