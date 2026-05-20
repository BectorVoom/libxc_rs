//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1482/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1482<F: Float>(t2439: F, t3895: F, t6919: F, t2453: F, t3908: F, t6889: F, t22398: F, t2470: F, t3915: F, t22452: F, t9680: F, t22409: F, t2435: F) -> (F, F, F, F, F) {
    let t73641 = t2439 * t3895 * t6919;
    let t73656 = t2453 * t6889 * t3908;
    let t73662 = t3915 * t22398 * t2470;
    let t73666 = t9680 * t22452 * t2470;
    let t73673 = t2435 * t22409;
    (t73641, t73656, t73662, t73666, t73673)
}
