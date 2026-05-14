//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 654/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk654<F: Float>(t1570: F, t165: F, t3188: F, t27420: F, t1360: F, t1642: F, t1557: F, t376: F, t7217: F, t1286: F, t7213: F, t497: F, t7166: F, t28: F, t108: F, t7211: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t27421 = t165 * t1570;
    let t27422 = t27421 * t3188;
    let t27423 = t27420 * t27422;
    let t27426 = t1642 * t1360;
    let t27427 = t165 * t1557;
    let t27428 = t27427 * t3188;
    let t27429 = t27426 * t27428;
    let t31995 = t376 * t7217;
    let t31997 = t1286 * t31995 / 9.0;
    let t31998 = t376 * t7213;
    let t32000 = t1286 * t31998 / 18.0;
    let t32001 = t7166 * t497;
    let t32002 = t28 * t32001;
    let t32011 = t7211 * t108;
    (t27422, t27423, t27426, t27428, t27429, t31995, t31997, t31998, t32000, t32001, t32002, t32011)
}
