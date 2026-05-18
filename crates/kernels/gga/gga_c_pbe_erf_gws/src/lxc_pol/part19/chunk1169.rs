//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1169/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1169<F: Float>(t14520: F, t14030: F, t14508: F, t14510: F, t14512: F, t14514: F, t14516: F, t14518: F, t14523: F, t14525: F, t15050: F, t14551: F) -> (F, F) {
    let t15057 = F::new(7.0) / F::new(144.0) * t14520;
    let t15060 = -t14030 + t15050 - t14508 / F::new(48.0) + t14510 / F::new(24.0) + t14512 / F::new(24.0) + t14514 / F::new(24.0) + F::new(5.0) / F::new(192.0) * t14516 + t14518 / F::new(96.0) - t15057 - t14523 / F::new(48.0) + t14525 / F::new(192.0);
    let t15070 = F::new(7.0) / F::new(576.0) * t14551;
    (t15060, t15070)
}
