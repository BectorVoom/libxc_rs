//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3114/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3114(t17303: f64, t3667: f64, t12886: f64, t5381: f64, t12627: f64, t489: f64, t17728: f64, t13011: f64, t5373: f64, t1222: f64, t5368: f64, t697: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57256 = t3667 * t17303;
    let t57257 = 0.14291339372689912324e-3_f64 * t57256;
    let t57258 = t5381 * t12886;
    let t57264 = t12627 * t489;
    let t57265 = t57264 * t17728;
    let t57270 = t5373 * t13011;
    let t57271 = t57270 / 162.0_f64;
    let t57273 = t1222 * t697 * t5368;
    (t57257, t57258, t57264, t57265, t57271, t57273)
}
