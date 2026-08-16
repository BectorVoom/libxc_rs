//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1224/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1224(t2934: f64, t633: f64, t1221: f64, t2146: f64, t2394: f64, t30023: f64, t32331: f64, t32335: f64, t32990: f64, t32992: f64, t32997: f64, t33000: f64, t33008: f64, t33751: f64, t35324: f64, t38015: f64, t38018: f64, t38019: f64, t38033: f64, t38036: f64, t8400: f64, t9427: f64) -> f64 {
    let t38040 = t2934 * t633;
    let t38044 = t38015 - 0.17347256376410398924e1_f64 * t32331 + t38018 + 0.13170898365871023197e1_f64 * t38019 + 0.17347256376410398924e1_f64 * t32335 - 0.34694512752820797848e1_f64 * t32990 - 0.65854491829355115987e0_f64 * t32992 + t32997 + 0.10408353825846239354e2_f64 * t33000 + 0.10408353825846239354e2_f64 * t2146 * t30023 * t2394 * t1221 + 0.13170898365871023197e1_f64 * t33008 - t38033 + t38036 - 0.26020884564615598386e1_f64 * t8400 * t9427 * t33751 + 0.26020884564615598386e1_f64 * t8400 * t38040 * t35324;
    t38044
}
