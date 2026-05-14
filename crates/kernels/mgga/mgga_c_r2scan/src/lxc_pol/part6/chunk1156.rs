//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1156/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1156<F: Float>(t1800: F, t1890: F, t5763: F, t1663: F, t390: F, t5791: F, t1422: F, t1793: F, t21066: F, t652: F, t1804: F, t5771: F, t5390: F, t5762: F, t5382: F, t5597: F) -> (F, F, F, F, F, F, F) {
    let t21094 = 0.66323093765092353863e3 * t1890 * t1800 * t5763;
    let t21097 = 0.12822e1 * t390 * t1663 * t5791;
    let t21098 = t1422 * t1793;
    let t21102 = 0.12372188467934141078e3 * t1890 * t21066 * t652;
    let t21104 = 0.76932e1 * t1804 * t5771;
    let t21107 = 0.11053848960848725644e3 * t390 * t5762 * t5390;
    let t21110 = 0.20620314113223568462e2 * t390 * t5597 * t5382;
    (t21094, t21097, t21098, t21102, t21104, t21107, t21110)
}
