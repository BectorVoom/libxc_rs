//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1358/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1358<F: Float>(t1161: F, t20154: F, t3067: F, t4207: F, t1109: F, t1205: F, t1185: F, t14887: F, t14888: F, t15035: F, t15536: F, t2376: F, t27105: F, t29775: F, t54998: F, t55005: F, t55007: F, t55022: F, t56250: F, t56255: F, t56265: F, t56269: F, t56276: F, t6793: F, t810: F, t8629: F, t8654: F, t8776: F) -> (F, F) {
    let t58047 = t20154 * t3067 * t4207 * t1161;
    let t58050 = t1205 * t1109;
    let t58065 = t56250 / F::new(192.0) + t29775 * t14888 / F::new(24.0) + F::new(5.0) / F::new(96.0) * t56255 - t54998 - t55005 - t55007 - t56265 / F::new(384.0) - t56269 / F::new(12.0) + t56276 / F::new(384.0) + t55022 - t6793 * t58047 / F::new(12.0) - t8629 * t20154 * t2376 * t58050 * t810 / F::new(48.0) + t8776 * t1185 * t15536 / F::new(96.0) + t8654 * t27105 * t14887 / F::new(24.0) + t8654 * t1185 * t15035 / F::new(24.0);
    (t58050, t58065)
}
