//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 768/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk768<F: Float>(t2185: F, t8675: F, t1997: F, t1986: F, t5277: F, t675: F, t1540: F, t880: F, t2141: F, t1347: F, t2406: F, t2392: F, t798: F, t26287: F, t4048: F, t30204: F) -> (F, F, F, F, F, F, F, F) {
    let t38967 = t8675 * t2185;
    let t38968 = t38967 * t1997;
    let t38969 = 0.24829349937757072982e-4 * t38968;
    let t38971 = t675 * t1986 * t5277;
    let t38973 = t1540 * t880;
    let t38974 = t38973 * t2141;
    let t38976 = t1347 * t2406;
    let t38977 = t2392 * t798;
    let t38978 = t26287 * t38977;
    let t38980 = t2392 * t4048;
    let t38981 = t30204 * t38980;
    (t38969, t38971, t38974, t38976, t38977, t38978, t38980, t38981)
}
