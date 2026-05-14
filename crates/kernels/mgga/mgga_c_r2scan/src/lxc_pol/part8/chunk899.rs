//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 899/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk899<F: Float>(t8088: F, t8089: F, t2155: F, t538: F, t920: F, t6194: F, t6191: F, t6212: F, t921: F, t6211: F, t6209: F, t6086: F, t6535: F, t2834: F, t780: F, t2106: F, t980: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8090 = t8088 * t8089;
    let t8092 = 0.19514881078765566037e-1 * t2155 * t8090;
    let t8123 = t538 * t920;
    let t8124 = t8123 * t6194;
    let t8125 = t6191 * t8124;
    let t8128 = t6212 * t921;
    let t8129 = t6211 * t8128;
    let t8130 = t6209 * t8129;
    let t8139 = t6086 * t8089;
    let t8141 = 0.23287303101564395622e-1 * t6535 * t8139;
    let t8146 = 0.23115257973478049502e0 * t2834 * t780;
    let t8147 = t980 * t2106;
    (t8090, t8092, t8123, t8124, t8125, t8128, t8129, t8130, t8139, t8141, t8146, t8147)
}
