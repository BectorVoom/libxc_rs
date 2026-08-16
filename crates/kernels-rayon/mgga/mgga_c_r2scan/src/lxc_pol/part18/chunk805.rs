//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 805/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk805(t8088: f64, t8089: f64, t2155: f64, t538: f64, t920: f64, t6194: f64, t6191: f64, t6212: f64, t921: f64, t6211: f64, t6209: f64, t6086: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8090 = t8088 * t8089;
    let t8092 = 0.19514881078765566037e-1_f64 * t2155 * t8090;
    let t8123 = t538 * t920;
    let t8124 = t8123 * t6194;
    let t8125 = t6191 * t8124;
    let t8128 = t6212 * t921;
    let t8129 = t6211 * t8128;
    let t8130 = t6209 * t8129;
    let t8139 = t6086 * t8089;
    (t8092, t8125, t8128, t8129, t8130, t8139)
}
