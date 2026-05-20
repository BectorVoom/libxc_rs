//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2754/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2754<F: Float>(t13725: F, t1904: F, t2439: F, t1364: F, t22441: F, t786: F, t22446: F, t2435: F, t14079: F, t14100: F, t3895: F, t6919: F) -> (F, F, F, F, F) {
    let t73593 = t2439 * t13725 * t1904;
    let t73598 = t786 * t22441 * t1364;
    let t73623 = t2435 * t22446;
    let t73627 = t14100 * t14079;
    let t73641 = t2439 * t3895 * t6919;
    (t73593, t73598, t73623, t73627, t73641)
}
