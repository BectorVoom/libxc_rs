//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1244/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1244<F: Float>(t2889: F, t300: F, t20636: F, t20641: F, t20647: F, t20649: F, t20652: F, t20654: F, t20658: F, t20662: F, t20665: F, t20667: F, t20670: F, t20674: F, t20676: F, t20678: F, t20685: F, t20687: F, t20693: F, t20695: F, t20697: F, t20824: F) -> (F, F) {
    let t21807 = t300 * t2889;
    let t21814 = -t20636 + t20641 + t20647 + t20649 + t20652 + t20654 - t20658 + t20662 - t20665 + t20667 - t20670 - t20674 + t20676 + t20678 - t20685 - t20687 - t20693 - t20695 - t20697 - t20824;
    (t21807, t21814)
}
