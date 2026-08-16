//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1011/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1011(t10185: f64, t10217: f64, t11285: f64, t11306: f64, t312: f64, t10015: f64, t10017: f64, t10018: f64, t10019: f64, t10022: f64, t10026: f64, t10248: f64, t10249: f64, t10250: f64, t4602: f64, t4652: f64, t4664: f64, t4744: f64, t4751: f64, t4784: f64, t4790: f64, t6076: f64, t7994: f64) -> f64 {
    let t11308 = t10185 + t10217 + t11285 + t11306;
    let t11309 = t11308 * t312;
    let t11310 = t10015 + t10017 + t4602 + t4744 + t4751 + t4652 - t7994 + t10018 + t4664 - t6076 + t10019 - t10022 - t10026 - t11309 - t4784 - t10248 - t4790 - t10249 + t10250;
    t11310
}
