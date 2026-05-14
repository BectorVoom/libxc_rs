//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 704/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk704<F: Float>(t2637: F, t4998: F, t2013: F, t5486: F, t6667: F, t5006: F, t2023: F, t2063: F, t5491: F, t1775: F, t5497: F, t2014: F, t220: F, t7246: F, t2643: F, t4419: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7602 = t4998 * t2637;
    let t7603 = t2013 * t7602;
    let t7605 = t5486 * t6667;
    let t7606 = t5006 * t7605;
    let t7609 = t2063 * t2023;
    let t7610 = t5491 * t7609;
    let t7611 = t1775 * t7610;
    let t7614 = t5497 * t6667;
    let t7615 = t1775 * t7614;
    let t7618 = t2014 * t220;
    let t7619 = t7246 * t7618;
    let t7624 = t4419 * t2643;
    (t7602, t7603, t7605, t7606, t7609, t7610, t7611, t7614, t7615, t7618, t7619, t7624)
}
