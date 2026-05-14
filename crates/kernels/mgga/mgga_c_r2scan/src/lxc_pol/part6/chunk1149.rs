//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1149/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1149<F: Float>(t565: F, t8028: F, t1610: F, t2207: F, t5162: F, t4933: F, t785: F, t788: F, t2214: F, t514: F, t5158: F, t2169: F, t6458: F, t1632: F, t551: F, t566: F) -> (F, F, F, F, F, F) {
    let t21003 = t565 * t8028;
    let t21009 = t2207 * t1610 * t5162;
    let t21013 = t2207 * t785 * t788 * t4933;
    let t21016 = t514 * t2214 * t5158;
    let t21018 = t2169 * t6458;
    let t21022 = t566 * t551 * t1632 * t4933;
    (t21003, t21009, t21013, t21016, t21018, t21022)
}
