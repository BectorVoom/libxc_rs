//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1306/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1306<F: Float>(t3604: F, t5490: F, t7227: F, t730: F, t21184: F, t7269: F, t1894: F, t9334: F, t1898: F, t3519: F, t1902: F, t1084: F, t21215: F, t2783: F, t7489: F, t2746: F, t7444: F) -> (F, F, F, F, F, F, F) {
    let t25824 = t5490 * t3604;
    let t25827 = 0.10254018858216406658e4 * t730 * t25824 * t7227;
    let t25829 = 24.0 * t21184 * t7269;
    let t25831 = 1.0 * t9334 * t1894;
    let t25832 = t3519 * t1898;
    let t25834 = 0.16081979498692535067e2 * t25832 * t1902;
    let t25836 = 2.0 * t21215 * t1084;
    let t25838 = 4.0 * t7489 * t2783;
    let t25840 = 2.0 * t2746 * t7444;
    (t25827, t25829, t25831, t25834, t25836, t25838, t25840)
}
