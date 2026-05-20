//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1508/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1508<F: Float>(t10858: F, t23257: F, t221: F, t23279: F, t10703: F, t2674: F, t2661: F, t2662: F, t6035: F, t61579: F, t1559: F, t18608: F) -> (F, F, F, F) {
    let t76596 = t10858 * t23257;
    let t76613 = t221 * t23279;
    let t76615 = t2674 * t10703 * t76613;
    let t76619 = t2661 * t2662 * t61579 * t6035;
    let t76645 = t2661 * t2662 * t18608 * t1559;
    (t76596, t76615, t76619, t76645)
}
