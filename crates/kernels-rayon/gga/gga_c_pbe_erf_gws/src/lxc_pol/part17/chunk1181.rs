//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1181/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1181(t14154: f64, t321: f64, t4058: f64, t6854: f64, t14157: f64, t13760: f64, t804: f64, t14150: f64, t353: f64, t4053: f64, t814: f64, t859: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t50837 = t321 * t14154;
    let t50839 = t4058 * t6854;
    let t50846 = t321 * t14157;
    let t50868 = t804 * t13760;
    let t50870 = t321 * t14150;
    let t50876 = t859 * t353 * t4053 * t814;
    (t50837, t50839, t50846, t50868, t50870, t50876)
}
