//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1793/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1793(t1012: f64, t1222: f64, t17401: f64, t1803: f64, t21017: f64, t24699: f64, t24706: f64, t24736: f64, t44959: f64, t484: f64, t59419: f64, t6690: f64, t70800: f64, t71928: f64, t71931: f64, t84082: f64, t84084: f64, t84195: f64, t87145: f64) -> f64 {
    let t91398 = -0.45732285992607719436e-2_f64 * t24699 * t1803 * t484 + 0.13719685797782315831e-1_f64 * t21017 * t24736 + 35.0_f64 / 972.0_f64 * t1222 * t1012 * t44959 * t87145 + t71928 / 216.0_f64 + t71931 / 108.0_f64 + 0.57927562257303111285e-1_f64 * t84082 + 0.57165357490759649296e-3_f64 * t84084 - 0.13550306960772657611e-2_f64 * t59419 - 0.25724410870841842184e-2_f64 * t70800 * t6690 - 0.25724410870841842184e-2_f64 * t17401 * t24706 - 7.0_f64 / 486.0_f64 * t84195;
    t91398
}
