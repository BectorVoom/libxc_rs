//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1005/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1005(t13850: f64, t520: f64, t1224: f64, t774: f64, t1206: f64, t5366: f64, t3348: f64, t3342: f64, t5420: f64, t10161: f64, t10166: f64, t1222: f64, t1244: f64, t12993: f64, t13004: f64, t13006: f64, t13013: f64, t13018: f64, t13021: f64, t13795: f64, t13800: f64) -> (f64, f64, f64, f64, f64) {
    let t13851 = t13850 * t520;
    let t13853 = t1224 * t774 * t13851;
    let t13856 = t5366 * t1206;
    let t13858 = t3348 * t774 * t13856;
    let t13862 = t3342 * t5420;
    let t13864 = t12993 - t13004 + t13006 - 35.0_f64 / 216.0_f64 * t10161 - t10166 - 5.0_f64 / 128.0_f64 * t1244 * t13795 + 5.0_f64 / 384.0_f64 * t1244 * t13800 - t1222 * t13853 / 3072.0_f64 + 5.0_f64 / 768.0_f64 * t1244 * t13858 + t13013 - 119.0_f64 / 1728.0_f64 * t13018 - 35.0_f64 / 1152.0_f64 * t13862 - t13021;
    (t13851, t13853, t13856, t13858, t13864)
}
