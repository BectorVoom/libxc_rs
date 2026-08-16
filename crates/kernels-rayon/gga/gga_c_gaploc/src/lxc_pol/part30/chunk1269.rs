//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1269/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1269(t11054: f64, t5640: f64, t24886: f64, t2660: f64, t10909: f64, t7416: f64, t25193: f64, t959: f64, t7482: f64, t8793: f64, t11005: f64, t2087: f64, t28129: f64, t28151: f64, t32923: f64, t32925: f64, t32927: f64, t32928: f64, t32931: f64, t32935: f64, t32936: f64, t4614: f64) -> f64 {
    let t32937 = t5640 * t11054;
    let t32938 = 0.1533717038156829987e1_f64 * t32937;
    let t32940 = 0.21450293971110256002e1_f64 * t24886 * t2660;
    let t32942 = 0.87421871174939309262e2_f64 * t7416 * t10909;
    let t32943 = t25193 * t959;
    let t32944 = 0.14896037479937677779e-1_f64 * t32943;
    let t32946 = 0.14300195980740170668e1_f64 * t8793 * t7482;
    let t32947 = t32923 + t32925 + t32927 + t28129 + t32928 + t32931 - 0.18404604457881959845e2_f64 * t2087 * t4614 * t11005 - t28151 + t32935 + t32936 + t32938 + t32940 + t32942 + t32944 + t32946;
    t32947
}
