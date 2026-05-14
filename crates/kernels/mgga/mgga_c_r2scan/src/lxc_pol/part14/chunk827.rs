//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 827/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk827<F: Float>(t2214: F, t2698: F, t514: F, t1616: F, t938: F, t2201: F, t785: F, t910: F, t2207: F, t2837: F, t783: F, t2842: F, t5100: F, t2832: F, t784: F, t788: F) -> (F, F, F, F, F, F) {
    let t8263 = t2214 * t2698;
    let t8265 = 0.19514881078765566037e-1 * t514 * t8263;
    let t8266 = t1616 * t938;
    let t8268 = t2201 * t785 * t8266;
    let t8270 = t1616 * t910;
    let t8272 = t2207 * t785 * t8270;
    let t8275 = t783 * t2837 * t1616;
    let t8277 = t5100 * t2842;
    let t8279 = t2832 * t784;
    let t8282 = 0.11643651550782197811e-1 * t783 * t8279 * t788;
    (t8265, t8268, t8272, t8275, t8277, t8282)
}
