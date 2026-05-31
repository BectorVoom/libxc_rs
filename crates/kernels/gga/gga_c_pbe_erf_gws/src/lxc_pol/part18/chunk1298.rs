//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1298/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1298<F: Float>(t11530: F, t50998: F, t51066: F, t15309: F, t51963: F, t4127: F, t8751: F, t13972: F, t15169: F, t1193: F, t14404: F, t14792: F, t29775: F, t3028: F, t3066: F, t353: F, t35566: F, t51509: F, t53034: F, t53042: F, t53047: F, t56520: F, t56525: F, t56534: F, t56545: F, t56548: F, t859: F, t8629: F, t8793: F) -> F {
    let t56551 = t50998 * t51066 * t11530;
    let t56553 = t51963 * t15309;
    let t56555 = t4127 * t8751;
    let t56560 = t13972 * t15169;
    let t56563 = t56520 / F::cast_from(1536.0_f64) + t29775 * t14404 / F::cast_from(24.0_f64) - t56525 / F::cast_from(1536.0_f64) + t8793 * t53034 / F::cast_from(24.0_f64) + t8793 * t53042 / F::cast_from(24.0_f64) + t8793 * t53047 / F::cast_from(24.0_f64) - t56534 / F::cast_from(768.0_f64) + t8629 * t859 * t353 * t1193 * t3028 / F::cast_from(96.0_f64) - t56545 / F::cast_from(384.0_f64) - t56548 / F::cast_from(768.0_f64) + t56551 / F::cast_from(192.0_f64) - F::cast_from(35.0_f64) / F::cast_from(1152.0_f64) * t56553 + t56555 / F::cast_from(48.0_f64) - t3066 * t35566 * t14792 / F::cast_from(8.0_f64) - F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t56560 - F::cast_from(119.0_f64) / F::cast_from(13824.0_f64) * t51509;
    t56563
}
