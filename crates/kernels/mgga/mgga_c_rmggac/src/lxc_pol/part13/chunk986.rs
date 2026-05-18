//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 986/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk986<F: Float>(t2085: F, t8339: F, t1162: F, t1979: F, t1982: F, t201: F, t589: F, t1692: F, t2046: F, t2050: F, t31: F, t2604: F, t8413: F) -> (F, F, F, F) {
    let t41656 = t8339 * t2085;
    let t41663 = t589 * t1162 * t201 * t1979 * t1982;
    let t41667 = t2046 * t2050 * t1692 * t31;
    let t41669 = t2604 * t8413;
    (t41656, t41663, t41667, t41669)
}
