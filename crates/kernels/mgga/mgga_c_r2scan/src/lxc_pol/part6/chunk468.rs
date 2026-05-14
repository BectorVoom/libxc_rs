//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 468/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk468<F: Float>(t1620: F, t507: F, t1551: F, t538: F, t529: F, t1554: F, t277: F, t502: F) -> (F, F, F, F) {
    let t1622 = 0.29272321618148349056e-1 * t1620 * t507;
    let t1624 = t538 * t1551;
    let t1625 = t529 * t1624;
    let t1628 = t538 * t1554;
    let t1629 = t529 * t1628;
    let t1632 = t502 * t277;
    (t1622, t1625, t1629, t1632)
}
