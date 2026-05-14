//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 638/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk638<F: Float>(t4744: F, t4746: F, t4748: F, t4751: F, t4733: F, t4736: F, t4739: F, t4849: F, t453: F, t1379: F, t445: F, t76: F, t1383: F, t84: F, t4811: F, t1481: F, t382: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4850 = 0.73355e-1 * t4744;
    let t4851 = 0.14671e0 * t4746;
    let t4852 = 0.17116166666666666667e0 * t4748;
    let t4853 = 0.36793333333333333333e0 * t4751;
    let t4854 = -0.34523333333333333333e1 * t4733 + 0.23015555555555555556e1 * t4736 - 0.26851481481481481482e1 * t4739 - t4849 + t4850 - t4851 - t4852 - t4853;
    let t4855 = t4854 * t453;
    let t4859 = 1.0 / t1379 / t445;
    let t4860 = t76 * t4859;
    let t4862 = 1.0 / t1383 / t84;
    let t4863 = t4811 * t4862;
    let t4867 = 1.0 / t1481 / t382;
    (t4850, t4851, t4852, t4853, t4854, t4855, t4859, t4860, t4862, t4863, t4867)
}
