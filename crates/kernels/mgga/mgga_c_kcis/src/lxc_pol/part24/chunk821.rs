//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 821/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk821<F: Float>(t1134: F, t18463: F, t1092: F, t6487: F, t9532: F, t13192: F, t4807: F, t2825: F, t6629: F, t1020: F, t2811: F, t6544: F) -> (F, F, F, F, F) {
    let t18464 = t18463 * t1134;
    let t18465 = t1092 * t18464;
    let t18467 = t9532 * t6487;
    let t18468 = t1092 * t18467;
    let t18471 = t13192 * t4807;
    let t18473 = t2825 * t6629;
    let t18474 = t1020 * t18473;
    let t18476 = t6544 * t2811;
    (t18465, t18468, t18471, t18474, t18476)
}
