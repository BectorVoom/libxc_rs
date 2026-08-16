//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1358/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1358(t1161: f64, t20154: f64, t3067: f64, t4207: f64, t1109: f64, t1205: f64, t1185: f64, t14887: f64, t14888: f64, t15035: f64, t15536: f64, t2376: f64, t27105: f64, t29775: f64, t54998: f64, t55005: f64, t55007: f64, t55022: f64, t56250: f64, t56255: f64, t56265: f64, t56269: f64, t56276: f64, t6793: f64, t810: f64, t8629: f64, t8654: f64, t8776: f64) -> (f64, f64) {
    let t58047 = t20154 * t3067 * t4207 * t1161;
    let t58050 = t1205 * t1109;
    let t58065 = t56250 / 192.0_f64 + t29775 * t14888 / 24.0_f64 + 5.0_f64 / 96.0_f64 * t56255 - t54998 - t55005 - t55007 - t56265 / 384.0_f64 - t56269 / 12.0_f64 + t56276 / 384.0_f64 + t55022 - t6793 * t58047 / 12.0_f64 - t8629 * t20154 * t2376 * t58050 * t810 / 48.0_f64 + t8776 * t1185 * t15536 / 96.0_f64 + t8654 * t27105 * t14887 / 24.0_f64 + t8654 * t1185 * t15035 / 24.0_f64;
    (t58050, t58065)
}
