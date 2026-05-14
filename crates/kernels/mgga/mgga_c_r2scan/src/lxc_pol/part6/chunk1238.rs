//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1238/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1238<F: Float>(t2132: F, t6527: F, t1616: F, t6416: F, t783: F, t19092: F, t507: F, t512: F, t1610: F, t5103: F, t5104: F, t119: F, t122: F, t19091: F, t1559: F, t283: F) -> (F, F, F, F, F, F) {
    let t23007 = t6527 * t2132;
    let t23014 = t783 * t6416 * t1616;
    let t23018 = 0.11535463037670312369e1 * t512 * t19092 * t507;
    let t23020 = t5103 * t1610 * t5104;
    let t23025 = 0.64878869728179484639e0 * t19091 * t119 * t122 * t507;
    let t23038 = t1559 * t1559;
    let t23040 = 1.0 / t283 / t23038;
    (t23007, t23014, t23018, t23020, t23025, t23040)
}
