//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1173/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1173<F: Float>(t121204: F, t13847: F, t1883: F, t121246: F, t121326: F, t32195: F, t32206: F, t5627: F, t9955: F, t125587: F, t32211: F, t3936: F) -> (F, F, F, F) {
    let t125796 = t13847 * t121204 * t1883;
    let t125797 = t121246 * t125796;
    let t125799 = t121326 * t125796;
    let t125803 = t32206 * t9955 * t32195 * t5627;
    let t125807 = t32206 * t3936 * t32211 * t125587;
    (t125797, t125799, t125803, t125807)
}
