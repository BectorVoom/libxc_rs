//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1244/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1244<F: Float>(t23214: F, t23215: F, t23216: F, t23218: F, t4936: F, t4961: F, t4968: F, t6598: F, t6602: F, t6606: F, t8506: F, t8507: F, t8509: F, t4980: F, t4797: F, t23219: F, t23221: F, t23225: F, t23226: F, t23230: F, t4977: F, t6773: F, t6957: F, t6969: F, t6972: F, t9907: F) -> (F, F) {
    let t23278 = t4936 + t6598 + t6602 + t6606 + 3.0 * t8506 + 6.0 * t8507 + 3.0 * t8509 + t4961 + t23214 - t23215 + t23216 + 0.97592231702715658578e-1 * t4968 + t23218;
    let t23281 = 48.0 * t4980;
    let t23283 = 0.34367190188705947438e1 * t4797;
    let t23285 = -t23219 - t4977 + 18.0 * t6957 + t9907 - t23221 - t23281 + t6773 + 6.0 * t6969 + t23225 - t23226 - t23283 + t23230 + 18.0 * t6972;
    (t23278, t23285)
}
