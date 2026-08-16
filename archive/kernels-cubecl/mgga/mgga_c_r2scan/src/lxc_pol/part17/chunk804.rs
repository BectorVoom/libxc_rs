//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 804/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk804<F: Float>(t8088: F, t8089: F, t2155: F, t538: F, t920: F, t6194: F, t6191: F, t6212: F, t921: F, t6211: F, t6209: F, t6086: F) -> (F, F, F, F, F, F) {
    let t8090 = t8088 * t8089;
    let t8092 = F::cast_from(0.19514881078765566037e-1_f64) * t2155 * t8090;
    let t8123 = t538 * t920;
    let t8124 = t8123 * t6194;
    let t8125 = t6191 * t8124;
    let t8128 = t6212 * t921;
    let t8129 = t6211 * t8128;
    let t8130 = t6209 * t8129;
    let t8139 = t6086 * t8089;
    (t8092, t8125, t8128, t8129, t8130, t8139)
}
