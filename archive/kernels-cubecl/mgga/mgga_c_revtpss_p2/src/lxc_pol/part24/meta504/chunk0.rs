//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1511/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1511<F: Float>(t2661: F, t2662: F, t4352: F, t6017: F, t23285: F, t2741: F, t23289: F, t6035: F, t61625: F, t23342: F, t2652: F, t221: F, t23114: F, t2674: F, t40683: F) -> (F, F, F, F, F, F) {
    let t76764 = t2661 * t2662 * t4352 * t6017;
    let t76767 = t2741 * t23285;
    let t76793 = t2741 * t23289;
    let t76797 = t2661 * t2662 * t61625 * t6035;
    let t76804 = t2652 * t23342;
    let t76808 = t2674 * t40683 * t221 * t23114;
    (t76764, t76767, t76793, t76797, t76804, t76808)
}
