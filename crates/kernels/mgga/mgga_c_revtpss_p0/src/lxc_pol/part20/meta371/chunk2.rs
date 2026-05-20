//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1350/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1350<F: Float>(t10696: F, t73: F, t2394: F, t2475: F, t2430: F, t10489: F, t10618: F, t10628: F, t10631: F, t10632: F, t10635: F, t14643: F, t225: F, t227: F, t229: F, t2634: F, t2638: F, t2639: F, t2642: F, t39476: F, t39736: F, t39751: F, t39787: F, t40089: F, t40123: F, t40152: F, t40180: F, t40213: F, t4415: F, t830: F, t832: F, t833: F) -> (F, F, F, F) {
    let t40231 = t73 * t10696;
    let t40232 = t2394 * t2394;
    let t40236 = t2475 * t2394;
    let t40240 = t2430 * t2430;
    let t40250 = -(t39736 + t39751 + t39787 + t40089 + t40123 + t40152 + t40180 + t40213) * t225 * t229 + F::new(12.0) * t10618 * t833 - F::new(72.0) * t2634 * t2639 + F::new(18.0) * t2634 * t2642 + F::new(240.0) * t830 * t10628 - F::new(144.0) * t14643 * t10632 + F::new(12.0) * t830 * t10635 - F::new(360.0) * t227 * t40231 * t40232 + F::new(360.0) * t4415 * t40236 * t2430 - F::new(36.0) * t227 * t2638 * t40240 - F::new(48.0) * t4415 * t10631 * t10489 + F::new(3.0) * t227 * t832 * t39476;
    (t40232, t40236, t40240, t40250)
}
