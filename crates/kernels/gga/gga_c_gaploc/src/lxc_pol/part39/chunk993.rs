//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 993/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk993<F: Float>(t40693: F, t40696: F, t40699: F, t40612: F, t40614: F, t40620: F, t40622: F, t40627: F, t40630: F, t40632: F, t40634: F, t471: F) -> (F, F, F, F) {
    let t43053 = F::cast_from(0.64087718584518535698e-3_f64) * t40693;
    let t43054 = F::cast_from(0.64087718584518535698e-3_f64) * t40696;
    let t43055 = F::cast_from(0.64087718584518535698e-3_f64) * t40699;
    let t43069 = (F::cast_from(21.0_f64) / F::cast_from(512.0_f64) * t40612 + F::cast_from(357.0_f64) / F::cast_from(16384.0_f64) * t40614 - F::cast_from(189.0_f64) / F::cast_from(262144.0_f64) * t40620 + F::cast_from(189.0_f64) / F::cast_from(0.16777216e8_f64) * t40622 - F::cast_from(63.0_f64) / F::cast_from(0.16777216e8_f64) * t40627 + F::cast_from(63.0_f64) / F::cast_from(262144.0_f64) * t40630 - F::cast_from(119.0_f64) / F::cast_from(16384.0_f64) * t40632 - F::cast_from(7.0_f64) / F::cast_from(512.0_f64) * t40634) * t471;
    (t43053, t43054, t43055, t43069)
}
