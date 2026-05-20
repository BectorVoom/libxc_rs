//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2699/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2699<F: Float>(t1448: F, t3829: F, t3889: F, t39989: F, t4139: F, t47086: F, t47088: F, t47092: F, t47096: F, t47098: F, t48305: F, t48307: F, t48308: F, t48311: F, t5542: F) -> (F, F) {
    let t49616 = t3829 * t1448;
    let t49630 = t3889 * t1448;
    let t49634 = -F::new(9.0) * t4139 * t49630 * t5542 - t39989 - t47086 + t47088 + t47092 - t47096 - t47098 + t48305 + t48307 + t48308 - t48311;
    (t49616, t49634)
}
