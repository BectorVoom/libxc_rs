//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 849/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk849<F: Float>(t597: F, t874: F, t10682: F, t10680: F, t2262: F, t2333: F, t2065: F, t3446: F, t3453: F, t2068: F, t120: F, t518: F, t3294: F) -> (F, F, F, F, F, F, F, F) {
    let t10683 = t597 * t874;
    let t10684 = t10682 * t10683;
    let t10685 = t10680 * t10684;
    let t10687 = t2333 * t2262;
    let t10692 = t3446 * t3453 * t2065;
    let t10695 = t3446 * t3453 * t2068;
    let t10697 = t120 * t518;
    let t10698 = t10697 * t3294;
    (t10683, t10684, t10685, t10687, t10692, t10695, t10697, t10698)
}
