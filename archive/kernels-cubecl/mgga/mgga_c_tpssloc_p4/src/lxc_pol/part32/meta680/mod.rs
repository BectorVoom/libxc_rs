//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta680 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2119;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2120;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta680<F: Float>(t12571: F, t24525: F, t27331: F, t9239: F, t2240: F, t27363: F, t33: F, t26012: F, t7255: F, t2109: F, t90090: F, t90094: F, t45844: F, t7245: F, t22550: F, t7974: F, t90247: F, t1419: F, t2274: F, t111: F, t27370: F, t2174: F, t5363: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t96028, t96045, t96072, t96102, t96110, t96115) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2119::<F>(t12571, t24525, t27331, t9239, t2240, t27363, t33, t26012, t7255, t2109, t90090, t90094);
        let (t96120, t96135, t96138, t96157, t96238, t96281) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2120::<F>(t45844, t7245, t22550, t7974, t2109, t90247, t1419, t2274, t111, t27370, t2174, t5363);
    (t96028, t96045, t96072, t96102, t96110, t96115, t96120, t96135, t96138, t96157, t96238, t96281)
}
