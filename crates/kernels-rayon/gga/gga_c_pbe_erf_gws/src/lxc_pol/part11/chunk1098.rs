//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1098/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1098(t30824: f64, t30839: f64, t16705: f64, t31785: f64, t31805: f64, t40163: f64, t40213: f64, t40251: f64, t47407: f64, t47412: f64, t47416: f64, t47470: f64, t47473: f64) -> (f64, f64, f64) {
    let t47586 = 32.0_f64 / 45.0_f64 * t30824;
    let t47587 = 8.0_f64 / 45.0_f64 * t30839;
    let t47598 = 0.50377777777777777778e-2_f64 * t31785 - 0.5037777777777777778e-2_f64 * t40213 + 0.15113333333333333333e-1_f64 * t40163 - t16705 + 0.33585185185185185186e-2_f64 * t31805 - 0.27987654320987654323e-2_f64 * t40251 + 0.45340000000000000001e-1_f64 * t47407 - 0.45340000000000000002e-1_f64 * t47470 + 0.37783333333333333335e-2_f64 * t47412 + 0.5037777777777777778e-2_f64 * t47473 - 0.4534e-1_f64 * t47416;
    (t47586, t47587, t47598)
}
