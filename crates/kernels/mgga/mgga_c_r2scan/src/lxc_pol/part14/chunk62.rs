//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 62/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk62<F: Float>(t149: F, t17: F, t19: F, t27: F) -> (F, F, F, F, F) {
    let t175 = F::cast_from(0.1898925e1_f64) * t149 + t17 + t19 + t27;
    let t178 = F::cast_from(1.0_f64) + F::cast_from(0.16081979498692535067e2_f64) / t175;
    let t179 = F::ln(t178);
    let t180 = F::cast_from(0.1328816518e-1_f64) * t179;
    let t181 = t175 * t175;
    let t182 = F::cast_from(1.0_f64) / t181;
    (t175, t178, t180, t181, t182)
}
