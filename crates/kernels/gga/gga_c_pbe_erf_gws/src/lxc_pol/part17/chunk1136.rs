//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1136/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1136<F: Float>(t1125: F, t14024: F, t3139: F, t9026: F, t4028: F, t14007: F, t3261: F, t14029: F, t14506: F, t14508: F, t14510: F, t14512: F, t14514: F, t14516: F, t14518: F) -> (F, F) {
    let t14520 = t1125 * t14024;
    let t14522 = t3139 * t9026;
    let t14523 = t4028 * t14522;
    let t14525 = t14007 * t3261;
    let t14527 = -F::new(7.0) / F::new(1152.0) * t14029 + F::new(7.0) / F::new(1152.0) * t14506 - t14508 / F::new(96.0) + t14510 / F::new(48.0) + t14512 / F::new(48.0) + t14514 / F::new(48.0) + F::new(5.0) / F::new(384.0) * t14516 + t14518 / F::new(192.0) - F::new(7.0) / F::new(288.0) * t14520 - t14523 / F::new(96.0) + t14525 / F::new(384.0);
    (t14522, t14527)
}
