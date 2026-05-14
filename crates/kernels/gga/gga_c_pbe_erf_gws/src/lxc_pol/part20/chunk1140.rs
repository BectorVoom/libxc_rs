//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1140/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1140<F: Float>(t15309: F, t51963: F, t4127: F, t8751: F, t13972: F, t15169: F, t1193: F, t14404: F, t14792: F, t29775: F, t3028: F, t3066: F, t353: F, t35566: F, t51509: F, t53034: F, t53042: F, t53047: F, t56520: F, t56525: F, t56534: F, t56545: F, t56548: F, t56551: F, t859: F, t8629: F, t8793: F) -> (F,) {
    let t56553 = t51963 * t15309;
    let t56555 = t4127 * t8751;
    let t56560 = t13972 * t15169;
    let t56563 = t56520 / 1536.0 + t29775 * t14404 / 24.0 - t56525 / 1536.0 + t8793 * t53034 / 24.0 + t8793 * t53042 / 24.0 + t8793 * t53047 / 24.0 - t56534 / 768.0 + t8629 * t859 * t353 * t1193 * t3028 / 96.0 - t56545 / 384.0 - t56548 / 768.0 + t56551 / 192.0 - 35.0 / 1152.0 * t56553 + t56555 / 48.0 - t3066 * t35566 * t14792 / 8.0 - 7.0 / 2304.0 * t56560 - 119.0 / 13824.0 * t51509;
    (t56563,)
}
