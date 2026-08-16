//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1036/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1036(t11207: f64, t895: f64, t904: f64, t912: f64, t1448: f64, t8772: f64, t2622: f64, t1411: f64, t2480: f64, t2483: f64, t10954: f64, t10956: f64, t10957: f64, t10963: f64, t10965: f64, t10968: f64, t10970: f64, t10972: f64, t11103: f64, t11123: f64, t11124: f64, t11146: f64, t11149: f64, t11155: f64, t11160: f64, t2807: f64, t2811: f64, t4023: f64, t4024: f64, t993: f64) -> (f64, f64, f64, f64) {
    let t11209 = t895 * t11207 * t904;
    let t11211 = 0.5848223622634646207e0_f64 * t912 * t11209;
    let t11212 = t8772 * t1448;
    let t11213 = t11212 * t2622;
    let t11215 = 0.10389515463408878255e3_f64 * t912 * t11213;
    let t11216 = t1411 * t2480;
    let t11218 = 2.0_f64 * t11216 * t2483;
    let t11219 = 2.0_f64 * t10957 * t2811 * t4023 - 2.0_f64 * t11124 * t4023 * t993 - t2807 * t4023 * t4024 + t10954 - t10956 + t10963 + t10965 + t10968 + t10970 + t10972 + t11103 + t11123 - t11146 - t11149 + t11155 - t11160 - t11211 + t11215 - t11218;
    (t11211, t11215, t11218, t11219)
}
