//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1796/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1796<F: Float>(t1234: F, t17307: F, t17934: F, t20850: F, t21541: F, t24633: F, t24713: F, t24934: F, t24941: F, t24956: F, t24974: F, t3670: F, t5326: F, t5486: F, t57465: F, t6573: F, t6717: F, t6720: F, t6723: F, t6738: F, t72267: F, t72326: F) -> F {
    let t91473 = F::cast_from(0.15805078039045227836e2_f64) * t17307 * t24941 + F::cast_from(0.79025390195226139183e1_f64) * t3670 * t21541 * t6573 - F::cast_from(0.39512695097613069592e1_f64) * t20850 * t6723 - F::cast_from(0.26341796731742046395e1_f64) * t1234 * t5486 * t24633 - F::cast_from(0.79025390195226139183e1_f64) * t5326 * t24934 + F::cast_from(0.15805078039045227836e2_f64) * t3670 * t5486 * t24713 - F::cast_from(0.39512695097613069592e1_f64) * t72326 * t6738 - F::cast_from(0.15805078039045227836e2_f64) * t57465 * t24956 + F::cast_from(0.15805078039045227836e2_f64) * t17934 * t24974 - F::cast_from(0.79025390195226139183e1_f64) * t20850 * t6720 - F::cast_from(0.79025390195226139183e1_f64) * t72267 * t6717;
    t91473
}
