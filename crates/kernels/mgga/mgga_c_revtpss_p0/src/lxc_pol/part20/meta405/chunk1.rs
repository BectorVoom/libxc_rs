//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1499/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1499<F: Float>(t1063: F, t247: F, t2853: F, t42447: F, t11151: F, t11725: F, t1042: F, t11653: F, t11714: F, t11748: F, t15716: F, t15728: F, t15935: F, t3101: F, t3116: F, t3127: F, t3130: F, t3182: F, t3188: F, t41277: F, t42001: F, t42421: F, t42425: F, t42428: F, t42439: F) -> F {
    let t42450 = t1063 * t247 * t42447 * t2853;
    let t42454 = t1063 * t247 * t11725 * t11151;
    let t42456 = -F::cast_from(0.1219527626469539185e-1_f64) * t42421 - F::cast_from(0.18292914397043087774e-1_f64) * t15728 * t11653 - F::cast_from(0.57927562257303111285e-1_f64) * t42425 * t3130 - F::cast_from(0.34299214494455789577e-2_f64) * t3127 * t1042 * t15935 * t42428 + F::cast_from(0.85748036236139473944e-2_f64) * t1063 * t247 * t3182 * t41277 + F::cast_from(0.18292914397043087774e-1_f64) * t11714 * t3101 - F::cast_from(0.22866142996303859718e-2_f64) * t42439 + F::cast_from(0.34299214494455789577e-2_f64) * t3188 * t11748 - F::cast_from(0.77173232612525526552e-2_f64) * t15716 * t247 * t3116 * t42001 - F::cast_from(0.31758531939310916276e-3_f64) * t42450 - F::cast_from(0.3811023832717309953e-2_f64) * t42454;
    t42456
}
