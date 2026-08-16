//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1440/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1440<F: Float>(t18562: F, t2496: F, t5825: F, t749: F, t2439: F, t6041: F, t780: F, t785: F, t18821: F, t2471: F, t18814: F, t2435: F) -> (F, F, F, F, F) {
    let t61296 = t18562 * t2496;
    let t61303 = t749 * t5825;
    let t61324 = t2439 * t785 * t6041 * t780;
    let t61330 = t18821 * t2471;
    let t61337 = t2435 * t18814;
    (t61296, t61303, t61324, t61330, t61337)
}
