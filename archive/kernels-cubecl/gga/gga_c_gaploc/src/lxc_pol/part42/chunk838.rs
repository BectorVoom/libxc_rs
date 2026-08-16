//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 838/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk838<F: Float>(t13552: F, t731: F, t13503: F, t2549: F, t40612: F, t40614: F, t40620: F, t40622: F, t40627: F, t40630: F, t40632: F, t40634: F, t471: F) -> (F, F, F, F) {
    let t44827 = t731 * t13552;
    let t44828 = F::cast_from(0.42725145723012357132e-3_f64) * t44827;
    let t44829 = t731 * t13503;
    let t44837 = t2549 * t13503;
    let t44855 = (F::cast_from(21.0_f64) / F::cast_from(256.0_f64) * t40612 + F::cast_from(357.0_f64) / F::cast_from(8192.0_f64) * t40614 - F::cast_from(189.0_f64) / F::cast_from(131072.0_f64) * t40620 + F::cast_from(189.0_f64) / F::cast_from(8388608.0_f64) * t40622 - F::cast_from(63.0_f64) / F::cast_from(8388608.0_f64) * t40627 + F::cast_from(63.0_f64) / F::cast_from(131072.0_f64) * t40630 - F::cast_from(119.0_f64) / F::cast_from(8192.0_f64) * t40632 - F::cast_from(7.0_f64) / F::cast_from(256.0_f64) * t40634) * t471;
    (t44828, t44829, t44837, t44855)
}
