//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 869/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk869<F: Float>(t40612: F, t40614: F, t40620: F, t40622: F, t40627: F, t40630: F, t40632: F, t40634: F, t471: F, t13516: F, t64: F, t11568: F, t871: F) -> (F, F, F) {
    let t44855 = (F::cast_from(21.0_f64) / F::cast_from(256.0_f64) * t40612 + F::cast_from(357.0_f64) / F::cast_from(8192.0_f64) * t40614 - F::cast_from(189.0_f64) / F::cast_from(131072.0_f64) * t40620 + F::cast_from(189.0_f64) / F::cast_from(8388608.0_f64) * t40622 - F::cast_from(63.0_f64) / F::cast_from(8388608.0_f64) * t40627 + F::cast_from(63.0_f64) / F::cast_from(131072.0_f64) * t40630 - F::cast_from(119.0_f64) / F::cast_from(8192.0_f64) * t40632 - F::cast_from(7.0_f64) / F::cast_from(256.0_f64) * t40634) * t471;
    let t44857 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t13516 * t64;
    let t44858 = t11568 * t871;
    (t44855, t44857, t44858)
}
