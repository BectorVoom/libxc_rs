//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 371/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk371<F: Float>(t24: F, t1246: F, t1256: F, t411: F, t415: F, t1173: F, t1187: F, t1213: F, t1215: F, t1219: F, t135: F, t273: F, t957: F, t1003: F, t148: F, t95: F, zeta_threshold: F) -> (F, F, F, F) {
    let t90 = t24 <= zeta_threshold;
    let t1259 = 0.65854491829355115987e0 * t1246 * t415 - 0.65854491829355115987e0 * t411 * t1256;
    let t1263 = t1259 * t135 * t273 * t957 - t1173 + t1187 + t1213 + t1215 - t1219;
    let t1265 = piecewise3(t90, 0.0, t1003);
    let t1281 = t148 * t95;
    (t1259, t1263, t1265, t1281)
}
