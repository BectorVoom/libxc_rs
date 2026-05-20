//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2217/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2217<F: Float>(t1479: F, t2282: F, t101303: F, t101376: F, t13312: F, t13392: F, t13396: F, t15936: F, t1923: F, t1927: F, t2122: F, t2123: F, t2251: F, t2258: F, t25117: F, t25146: F, t25150: F, t26776: F, t26783: F, t26786: F, t26789: F, t29355: F, t29363: F, t29364: F, t29367: F, t6954: F, t6977: F, t72: F, t7571: F, t7702: F, t8143: F, t8144: F, t8147: F, t92612: F, t96733: F) -> F {
    let t104379 = t1479 * t2282;
    let t104403 = t25117 * t8147 / F::new(3.0) + t101376 * t2123 / F::new(3.0) - t1923 * t2122 * t101303 / F::new(6.0) - t7702 * t26783 / F::new(6.0) - t7702 * t26786 / F::new(3.0) - t7702 * t26789 / F::new(6.0) - t25150 * t8144 / F::new(6.0) - t6954 * t29364 / F::new(3.0) - t6954 * t29367 / F::new(3.0) - t1923 * (-F::new(20.0) / F::new(27.0) * t104379 * t2251 + F::new(20.0) / F::new(9.0) * t29355 * t2258 + F::new(5.0) / F::new(108.0) * t96733 * t15936 + F::new(5.0) / F::new(9.0) * t26776 * t13396 + F::new(5.0) / F::new(18.0) * t26776 * t13392 - F::new(5.0) / F::new(6.0) * t7571 * t13312 + t92612) * t72 * t1927 / F::new(6.0) - t1923 * t29363 * t6977 / F::new(3.0) - t1923 * t8143 * t25146 / F::new(6.0);
    t104403
}
