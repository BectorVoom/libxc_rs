//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1702/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1702(t26284: f64, t26285: f64, t22844: f64, t6604: f64, t1361: f64, t5308: f64, t1339: f64, t5287: f64, t6936: f64, t22779: f64, t7712: f64, t16225: f64, t550: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
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
