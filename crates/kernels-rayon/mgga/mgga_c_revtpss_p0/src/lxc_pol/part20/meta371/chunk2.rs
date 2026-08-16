//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1350/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1350(t10696: f64, t73: f64, t2394: f64, t2475: f64, t2430: f64, t10489: f64, t10618: f64, t10628: f64, t10631: f64, t10632: f64, t10635: f64, t14643: f64, t225: f64, t227: f64, t229: f64, t2634: f64, t2638: f64, t2639: f64, t2642: f64, t39476: f64, t39736: f64, t39751: f64, t39787: f64, t40089: f64, t40123: f64, t40152: f64, t40180: f64, t40213: f64, t4415: f64, t830: f64, t832: f64, t833: f64) -> (f64, f64, f64, f64) {
    let t40231 = t73 * t10696;
    let t40232 = t2394 * t2394;
    let t40236 = t2475 * t2394;
    let t40240 = t2430 * t2430;
    let t40250 = -(t39736 + t39751 + t39787 + t40089 + t40123 + t40152 + t40180 + t40213) * t225 * t229 + 12.0_f64 * t10618 * t833 - 72.0_f64 * t2634 * t2639 + 18.0_f64 * t2634 * t2642 + 240.0_f64 * t830 * t10628 - 144.0_f64 * t14643 * t10632 + 12.0_f64 * t830 * t10635 - 360.0_f64 * t227 * t40231 * t40232 + 360.0_f64 * t4415 * t40236 * t2430 - 36.0_f64 * t227 * t2638 * t40240 - 48.0_f64 * t4415 * t10631 * t10489 + 3.0_f64 * t227 * t832 * t39476;
    (t40232, t40236, t40240, t40250)
}
