//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2571/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2571(t247: f64, t44545: f64, t5230: f64, t5384: f64, t12984: f64, t5327: f64, t17303: f64, t3667: f64, t12627: f64, t489: f64, t17728: f64, t13011: f64, t5373: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57241 = t5384 * t247 * t44545 * t5230;
    let t57242 = 0.28582678745379824648e-3_f64 * t57241;
    let t57250 = t5327 * t12984;
    let t57251 = 0.14291339372689912324e-3_f64 * t57250;
    let t57256 = t3667 * t17303;
    let t57257 = 0.14291339372689912324e-3_f64 * t57256;
    let t57264 = t12627 * t489;
    let t57265 = t57264 * t17728;
    let t57270 = t5373 * t13011;
    (t57242, t57251, t57257, t57264, t57265, t57270)
}
