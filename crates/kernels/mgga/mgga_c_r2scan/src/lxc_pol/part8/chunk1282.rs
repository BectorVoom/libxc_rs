//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1282/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1282<F: Float>(t2148: F, t22790: F, t30057: F, t6465: F, t8866: F, t2155: F, t22962: F, t29837: F, t2214: F, t514: F, t9301: F, t1620: F, t9553: F, t5100: F, t9236: F, t9273: F) -> (F, F, F, F, F, F, F) {
    let t30059 = t22790 * t2148 * t30057;
    let t30069 = t6465 * t8866;
    let t30072 = t2155 * t22962 * t29837;
    let t30092 = t514 * t2214 * t9301;
    let t30094 = t1620 * t9553;
    let t30098 = t5100 * t9236;
    let t30100 = t5100 * t9273;
    (t30059, t30069, t30072, t30092, t30094, t30098, t30100)
}
