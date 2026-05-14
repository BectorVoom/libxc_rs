//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 508/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk508<F: Float>(t1800: F, t182: F, t189: F, t1647: F, t649: F, t652: F, t1416: F, t230: F, t1691: F, t225: F, t748: F) -> (F, F, F, F, F, F) {
    let t1803 = 0.2137e0 * t182 * t1800 * t189;
    let t1804 = t649 * t1647;
    let t1806 = 0.34367190188705947438e1 * t1804 * t652;
    let t1808 = 20.0 * t1416 * t230;
    let t1809 = t225 * t1691;
    let t1810 = t748 * t1809;
    (t1803, t1804, t1806, t1808, t1809, t1810)
}
