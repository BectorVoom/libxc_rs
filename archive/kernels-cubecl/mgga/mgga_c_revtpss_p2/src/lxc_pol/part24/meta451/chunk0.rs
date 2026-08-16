//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1415/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1415<F: Float>(t14362: F, t9863: F, t9866: F, t10115: F, t1570: F, t4322: F, t9292: F, t10981: F, t1579: F, t22: F, t868: F, t2465: F, t4480: F, t9288: F) -> (F, F, F, F, F, F) {
    let t50092 = t14362 * t9863;
    let t50094 = t14362 * t9866;
    let t50155 = t10115 * t1570;
    let t50166 = t9292 * t4322;
    let t50178 = t10981 * t868 * t1579 * t22;
    let t50205 = t2465 * t4480 * t9288;
    (t50092, t50094, t50155, t50166, t50178, t50205)
}
