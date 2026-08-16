//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1702/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1702<F: Float>(t26284: F, t26285: F, t22844: F, t6604: F, t1361: F, t5308: F, t1339: F, t5287: F, t6936: F, t22779: F, t7712: F, t16225: F, t550: F) -> (F, F, F, F, F, F, F, F) {
    let t26286 = t26284 * t26285;
    let t26288 = t22844 * t6604;
    let t26289 = t1361 * t5308;
    let t26290 = t26288 * t26289;
    let t26292 = t1339 * t5287;
    let t26293 = t6936 * t26292;
    let t26295 = t22779 * t7712;
    let t26297 = t16225 * t550;
    (t26286, t26288, t26289, t26290, t26292, t26293, t26295, t26297)
}
