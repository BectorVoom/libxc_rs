//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 848/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk848(t1621: f64, t7160: f64, t639: f64, t1724: f64, t2601: f64, t2598: f64, t4913: f64, t172: f64, t2824: f64, t184: f64, t564: f64, t1872: f64, t2790: f64) -> (f64, f64, f64, f64, f64) {
    let t7161 = t1621 * t7160;
    let t7163 = 8.0_f64 / 15.0_f64 * t639 * t7161;
    let t7164 = t2601 * t1724;
    let t7165 = t1621 * t7164;
    let t7167 = 4.0_f64 / 15.0_f64 * t639 * t7165;
    let t7169 = 8.0_f64 / 15.0_f64 * t4913 * t2598;
    let t7170 = t172 * t2824;
    let t7171 = t7170 * t184;
    let t7173 = 8.0_f64 / 15.0_f64 * t7171 * t564;
    let t7175 = 4.0_f64 / 15.0_f64 * t2790 * t1872;
    (t7163, t7167, t7169, t7173, t7175)
}
