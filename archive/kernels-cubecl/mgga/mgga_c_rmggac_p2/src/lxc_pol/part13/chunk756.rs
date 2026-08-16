//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 756/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk756<F: Float>(t2064: F, t3928: F, t798: F, t1550: F, t4048: F, t7778: F, t2084: F, t27: F, t7273: F, t839: F, t118: F, t1986: F, t209: F, t35192: F) -> (F, F, F, F) {
    let t35407 = t3928 * t2064 * t798;
    let t35413 = t1550 * t7778 * t4048;
    let t35424 = t7273 * t27 * t2084 * t839;
    let t35455 = t1986 * t118 * t35192 * t209;
    (t35407, t35413, t35424, t35455)
}
