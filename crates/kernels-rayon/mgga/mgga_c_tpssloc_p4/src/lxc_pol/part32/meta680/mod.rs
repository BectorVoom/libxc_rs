//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta680 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2119;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2120;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta680(t12571: f64, t24525: f64, t27331: f64, t9239: f64, t2240: f64, t27363: f64, t33: f64, t26012: f64, t7255: f64, t2109: f64, t90090: f64, t90094: f64, t45844: f64, t7245: f64, t22550: f64, t7974: f64, t90247: f64, t1419: f64, t2274: f64, t111: f64, t27370: f64, t2174: f64, t5363: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t96028, t96045, t96072, t96102, t96110, t96115) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2119(t12571, t24525, t27331, t9239, t2240, t27363, t33, t26012, t7255, t2109, t90090, t90094);
        let (t96120, t96135, t96138, t96157, t96238, t96281) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2120(t45844, t7245, t22550, t7974, t2109, t90247, t1419, t2274, t111, t27370, t2174, t5363);
    (t96028, t96045, t96072, t96102, t96110, t96115, t96120, t96135, t96138, t96157, t96238, t96281)
}
