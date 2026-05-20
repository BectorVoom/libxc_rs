//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2800/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2800<F: Float>(t2289: F, t5892: F, t21821: F, t625: F, t21824: F, t1455: F, t6951: F, t1464: F, t6936: F, t22571: F, t571: F, t25048: F, t575: F) -> (F, F, F, F, F, F, F) {
    let t75639 = t2289 * t5892;
    let t75641 = t625 * t21821;
    let t75643 = t625 * t21824;
    let t75720 = t1455 * t6951;
    let t75727 = t6936 * t1464;
    let t75796 = t571 * t22571;
    let t75808 = t25048 * t575;
    (t75639, t75641, t75643, t75720, t75727, t75796, t75808)
}
