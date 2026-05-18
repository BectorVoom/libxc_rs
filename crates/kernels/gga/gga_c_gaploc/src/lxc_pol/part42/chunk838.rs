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
    let t44828 = F::new(0.42725145723012357132e-3) * t44827;
    let t44829 = t731 * t13503;
    let t44837 = t2549 * t13503;
    let t44855 = (F::new(21.0) / F::new(256.0) * t40612 + F::new(357.0) / F::new(8192.0) * t40614 - F::new(189.0) / F::new(131072.0) * t40620 + F::new(189.0) / F::new(8388608.0) * t40622 - F::new(63.0) / F::new(8388608.0) * t40627 + F::new(63.0) / F::new(131072.0) * t40630 - F::new(119.0) / F::new(8192.0) * t40632 - F::new(7.0) / F::new(256.0) * t40634) * t471;
    (t44828, t44829, t44837, t44855)
}
