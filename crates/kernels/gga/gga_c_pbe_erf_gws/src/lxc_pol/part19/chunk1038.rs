//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1038/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1038<F: Float>(t14003: F, t14115: F, t14338: F, t14345: F, t14755: F, t14768: F, t14773: F, t14777: F, t14782: F, t14785: F, t14788: F, t15004: F, t335: F, t4083: F, t8654: F, t1206: F, t3200: F, t338: F) -> (F, F) {
    let t15016 = t14755 / 768.0 - t335 * t15004 / 96.0 + t14338 + t14003 + t14115 + t14768 / 48.0 - t14773 / 24.0 - 7.0 / 144.0 * t14345 + t14777 / 768.0 - t14782 / 48.0 - t14785 / 192.0 - t14788 / 48.0 - t8654 * t4083 / 96.0;
    let t15018 = t338 * t3200 * t1206;
    (t15016, t15018)
}
