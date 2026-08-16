//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1165/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1165<F: Float>(t338: F, t4228: F, t892: F, t14003: F, t14115: F, t14338: F, t14345: F, t14755: F, t14768: F, t14773: F, t14777: F, t14782: F, t14785: F, t14788: F, t335: F, t4083: F, t8654: F) -> (F, F) {
    let t15004 = t338 * t892 * t4228;
    let t15016 = t14755 / F::cast_from(768.0_f64) - t335 * t15004 / F::cast_from(96.0_f64) + t14338 + t14003 + t14115 + t14768 / F::cast_from(48.0_f64) - t14773 / F::cast_from(24.0_f64) - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t14345 + t14777 / F::cast_from(768.0_f64) - t14782 / F::cast_from(48.0_f64) - t14785 / F::cast_from(192.0_f64) - t14788 / F::cast_from(48.0_f64) - t8654 * t4083 / F::cast_from(96.0_f64);
    (t15004, t15016)
}
