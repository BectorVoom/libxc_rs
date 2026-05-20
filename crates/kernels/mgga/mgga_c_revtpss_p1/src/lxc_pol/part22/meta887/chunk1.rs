//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3075/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3075<F: Float>(t2873: F, t4587: F, t11298: F, t1596: F, t11466: F, t1633: F, t11299: F, t1609: F, t15494: F, t964: F, t3011: F, t4682: F) -> (F, F, F, F, F, F) {
    let t52505 = t4587 * t2873;
    let t52508 = t1596 * t11298;
    let t52511 = t11466 * t1633;
    let t52514 = t11299 * t1609;
    let t52522 = t15494 * t964;
    let t52637 = t4682 * t3011;
    (t52505, t52508, t52511, t52514, t52522, t52637)
}
