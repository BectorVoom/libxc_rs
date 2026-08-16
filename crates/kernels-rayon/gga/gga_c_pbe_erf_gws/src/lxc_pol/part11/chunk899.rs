//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 899/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk899(t265: f64, t266: f64, t837: f64, t245: f64, t5420: f64, t712: f64, t5427: f64, t723: f64, t1903: f64, t1924: f64, t1917: f64, t703: f64) -> (f64, f64, f64, f64, f64) {
    let t18280 = 56.0_f64 / 1215.0_f64 * t265 * t266 * t837;
    let t18309 = 0.2e-20_f64 * t712 * t245 * t5420;
    let t18311 = 8.0_f64 / 9.0_f64 * t5427 * t723;
    let t18315 = 4.0_f64 / 9.0_f64 * t1924 * t1903;
    let t18318 = 0.5402469135802469136e-1_f64 * t712 * t703 * t1917;
    (t18280, t18309, t18311, t18315, t18318)
}
