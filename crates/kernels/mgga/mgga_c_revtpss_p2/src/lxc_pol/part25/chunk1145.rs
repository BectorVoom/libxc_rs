//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1145/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1145<F: Float>(t25479: F, t7160: F, t1976: F, t3075: F, t7145: F, t1982: F, t3259: F, t1972: F, t3223: F, t1024: F, t7125: F, t3215: F, t7117: F) -> (F, F, F, F, F, F, F) {
    let t25480 = t7160 * t25479;
    let t25483 = t1976 * t3075;
    let t25484 = t7145 * t25483;
    let t25487 = t1982 * t3259;
    let t25490 = t3223 * t1972;
    let t25495 = t1024 * t7125;
    let t25498 = t7117 * t3215;
    (t25480, t25483, t25484, t25487, t25490, t25495, t25498)
}
