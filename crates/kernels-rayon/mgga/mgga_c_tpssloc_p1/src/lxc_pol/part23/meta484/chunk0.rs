//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1472/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1472(t225: f64, t78637: f64, t11546: f64, t1174: f64, t15569: f64, t15740: f64, t1653: f64, t1726: f64, t22162: f64, t22244: f64, t22280: f64, t22288: f64, t3440: f64, t3577: f64, t3578: f64, t45112: f64, t484: f64, t488: f64, t52628: f64, t52879: f64, t53274: f64, t66500: f64, t68: f64, t73043: f64, t73113: f64, t78035: f64, t78039: f64) -> (f64, f64) {
    let t79260 = t78637 * t225;
    let t79282 = -t15740 * t22288 / 192.0_f64 + t52628 * t22280 / 36.0_f64 - t52879 * t22280 / 192.0_f64 - t45112 + t79260 * t68 * t484 * t488 / 3072.0_f64 - 7.0_f64 / 108.0_f64 * t1174 * t11546 * t78035 + 154.0_f64 / 243.0_f64 * t73113 * t1726 + t1174 * t3440 * t78039 / 6.0_f64 - t53274 / 486.0_f64 + t73043 / 1152.0_f64 - t3577 * t3578 * t22244 * t1653 / 1152.0_f64 + t15569 * t22162 / 72.0_f64 - 11.0_f64 / 81.0_f64 * t66500;
    (t79260, t79282)
}
