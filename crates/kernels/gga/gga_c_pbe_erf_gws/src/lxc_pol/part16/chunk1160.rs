//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1160/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1160<F: Float>(t338: F, t4228: F, t892: F, t14003: F, t14115: F, t14338: F, t14345: F, t14755: F, t14768: F, t14773: F, t14777: F, t14782: F, t14785: F, t14788: F, t335: F, t4083: F, t8654: F) -> (F, F) {
    let t15004 = t338 * t892 * t4228;
    let t15016 = t14755 / F::new(768.0) - t335 * t15004 / F::new(96.0) + t14338 + t14003 + t14115 + t14768 / F::new(48.0) - t14773 / F::new(24.0) - F::new(7.0) / F::new(144.0) * t14345 + t14777 / F::new(768.0) - t14782 / F::new(48.0) - t14785 / F::new(192.0) - t14788 / F::new(48.0) - t8654 * t4083 / F::new(96.0);
    (t15004, t15016)
}
