//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1059/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1059<F: Float>(t32629: F, t7238: F, t2014: F, t2107: F, t32113: F, t7235: F, t8718: F, t7536: F, t8717: F, t2007: F, t2052: F, t2108: F, t32322: F, t32619: F, t32620: F, t32621: F, t32627: F, t32628: F, t651: F, t7221: F, t7357: F, t7537: F, t7539: F, t8568: F) -> (F, F, F, F) {
    let t32630 = t32629 * t7238;
    let t32632 = F::cast_from(3.0_f64) * t2014 * t32630;
    let t32633 = t2107 * t32113;
    let t32634 = t2014 * t32633;
    let t32635 = t7235 * t8718;
    let t32636 = t7536 * t8717;
    let t32637 = t2014 * t32636;
    let t32638 = -t2007 * t7357 - t2052 * t7221 + t2108 * t32322 - F::cast_from(2.0_f64) * t32621 * t651 + t7537 * t8568 - t7539 * t8568 - t32619 - t32620 + t32627 + t32628 + t32632 - t32634 - t32635 - t32637;
    (t32630, t32633, t32636, t32638)
}
