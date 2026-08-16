//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1523/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1523<F: Float>(t19804: F, t562: F, t1372: F, t6361: F, t225: F, t6435: F, t1323: F, t6434: F, t1385: F, t6439: F, t12021: F, t6362: F) -> (F, F, F, F, F, F, F) {
    let t20038 = t19804 * t562;
    let t20040 = t6361 * t1372;
    let t20044 = t6435 * t225;
    let t20048 = t1323 * t6434;
    let t20050 = t6439 * t1385;
    let t20051 = t12021 * t20050;
    let t20060 = t6362 * t225;
    (t20038, t20040, t20044, t20048, t20050, t20051, t20060)
}
