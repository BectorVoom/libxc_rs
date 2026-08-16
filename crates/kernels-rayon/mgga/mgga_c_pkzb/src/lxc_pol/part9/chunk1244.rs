//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1244/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1244(t2889: f64, t300: f64, t20636: f64, t20641: f64, t20647: f64, t20649: f64, t20652: f64, t20654: f64, t20658: f64, t20662: f64, t20665: f64, t20667: f64, t20670: f64, t20674: f64, t20676: f64, t20678: f64, t20685: f64, t20687: f64, t20693: f64, t20695: f64, t20697: f64, t20824: f64) -> (f64, f64) {
    let t21807 = t300 * t2889;
    let t21814 = -t20636 + t20641 + t20647 + t20649 + t20652 + t20654 - t20658 + t20662 - t20665 + t20667 - t20670 - t20674 + t20676 + t20678 - t20685 - t20687 - t20693 - t20695 - t20697 - t20824;
    (t21807, t21814)
}
