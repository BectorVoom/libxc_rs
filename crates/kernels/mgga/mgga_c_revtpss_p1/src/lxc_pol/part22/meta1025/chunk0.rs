//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3586/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3586<F: Float>(t20267: F, t698: F, t1145: F, t141: F, t68273: F, t2258: F, t6421: F, t68269: F, t20297: F, t3417: F, t20292: F, t2251: F) -> (F, F, F, F, F, F, F, F) {
    let t68312 = t698 * t20267;
    let t68315 = t141 * t1145 * t68273;
    let t68317 = t6421 * t2258;
    let t68319 = t141 * t1145 * t68317;
    let t68322 = t141 * t1145 * t68269;
    let t68324 = t20297 * t2258;
    let t68326 = t141 * t3417 * t68324;
    let t68328 = t20292 * t2251;
    (t68312, t68315, t68317, t68319, t68322, t68324, t68326, t68328)
}
