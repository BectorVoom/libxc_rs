//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2260/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2260<F: Float>(t28019: F, t531: F, t2014: F, t7238: F, t25866: F, t7898: F, t13867: F, t28167: F, t8996: F, t13872: F, t100940: F, t101120: F, t101124: F, t101407: F, t101416: F, t118: F, t1310: F, t14310: F, t1843: F, t2011: F, t25169: F, t25872: F, t28160: F, t4151: F, t4248: F, t508: F, t5517: F, t5787: F, t6983: F, t7231: F, t7894: F, t98615: F, t98617: F, t98621: F, t98623: F) -> F {
    let t101417 = t531 * t28019;
    let t101420 = F::cast_from(6.0_f64) * t2014 * t101417 * t7238;
    let t101422 = F::cast_from(6.0_f64) * t7898 * t25866;
    let t101428 = F::cast_from(12.0_f64) * t28167 * t8996 * t13867;
    let t101431 = F::cast_from(6.0_f64) * t28167 * t8996 * t13872;
    let t101432 = -t98615 - t98617 + t98621 - t98623 - F::cast_from(4.0_f64) * t4248 * t25872 + t2011 * t14310 - t118 * (t100940 + t101120) - t101124 - t101407 * t508 - F::cast_from(2.0_f64) * t28160 * t1310 - t25169 * t1843 - F::cast_from(2.0_f64) * t6983 * t5517 + t101416 + t101420 + t101422 + t7894 * t4151 + F::cast_from(2.0_f64) * t7231 * t5787 + t101428 + t101431;
    t101432
}
