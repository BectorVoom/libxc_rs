//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 921/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk921<F: Float>(t2317: F, t58: F, t423: F, t597: F, t874: F, t10680: F, t120: F, t518: F, t3294: F) -> (F, F, F, F, F, F, F) {
    let t10681 = t2317 * t58;
    let t10682 = t10681 * t423;
    let t10683 = t597 * t874;
    let t10684 = t10682 * t10683;
    let t10685 = t10680 * t10684;
    let t10697 = t120 * t518;
    let t10698 = t10697 * t3294;
    (t10681, t10682, t10683, t10684, t10685, t10697, t10698)
}
