//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1205/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1205<F: Float>(t24776: F, t24176: F, t6462: F, t2110: F, t22: F, t3436: F, t6: F, t6068: F, t923: F, t20994: F, t2547: F, t20150: F, t7460: F, t2526: F, t551: F, t566: F, t6343: F) -> (F, F, F, F, F, F) {
    let t24777 = 0.12713391885412927226e1 * t24776;
    let t24804 = t6462 * t24176;
    let t24805 = 0.86743646395112941037e-3 * t24804;
    let t24822 = t22 * t6 * t3436 * t2110 * t6068 * t923;
    let t24838 = t20994 * t2547;
    let t24839 = 0.12805040077930161442e1 * t24838;
    let t24840 = t20150 * t7460;
    let t24858 = t566 * t551 * t6343 * t2526;
    (t24777, t24805, t24822, t24839, t24840, t24858)
}
