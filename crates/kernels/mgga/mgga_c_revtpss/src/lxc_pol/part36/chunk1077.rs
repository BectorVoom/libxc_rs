//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1077/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1077<F: Float>(t1916: F, t7953: F, t1936: F, t5883: F, t572: F, t1518: F, t28276: F, t5920: F, t7330: F, t117: F, t30004: F, t1469: F, t25137: F, t26776: F, t29355: F, t5819: F, t5825: F, t5842: F, t61: F, t7571: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t30184 = 6.0 * t1916 * t7953;
    let t30185 = t5883 * t1936;
    let t30187 = 6.0 * t572 * t30185;
    let t30188 = t28276 * t1518;
    let t30190 = 12.0 * t572 * t30188;
    let t30191 = t7330 * t5920;
    let t30193 = 6.0 * t572 * t30191;
    let t30194 = t117 * t30004;
    let t30196 = 3.0 * t572 * t30194;
    let t30681 = 88.0 / 9.0 * t5842 * t61 + 40.0 / 9.0 * t29355 * t1469 + 5.0 / 18.0 * t26776 * t5819 - 5.0 / 6.0 * t7571 * t5825 - t25137;
    (t30184, t30185, t30187, t30188, t30190, t30191, t30193, t30194, t30196, t30681)
}
