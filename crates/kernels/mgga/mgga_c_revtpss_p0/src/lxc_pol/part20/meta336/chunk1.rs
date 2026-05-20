//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1260/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1260<F: Float>(t241: F, t820: F, t9991: F, t2482: F, t4000: F, t814: F, t136: F, t550: F, t220: F, t1392: F, t73: F, t844: F) -> (F, F, F, F, F) {
    let t13804 = t820 * t9991 * t241;
    let t13845 = t2482 * t4000 * t814;
    let t13846 = t550 * t136;
    let t13847 = t13846 * t220;
    let t13902 = t1392 * t73;
    let t13999 = t820 * t4000 * t844;
    (t13804, t13845, t13847, t13902, t13999)
}
