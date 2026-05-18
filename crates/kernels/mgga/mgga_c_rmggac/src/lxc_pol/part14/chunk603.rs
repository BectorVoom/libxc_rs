//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 603/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk603<F: Float>(t7206: F, t7788: F, t305: F, t7779: F, t7769: F, t797: F, t7578: F, t321: F, t664: F, t333: F, t352: F, t645: F, t833: F) -> (F, F, F, F, F, F, F, F) {
    let t7789 = t7788 * t7206;
    let t7793 = t305 * t7779;
    let t7795 = t797 * t7769;
    let t7796 = F::new(0.23948483403727617128e0) * t7795;
    let t7797 = t305 * t7578;
    let t7799 = t664 * t321;
    let t7800 = t7799 * t333;
    let t7803 = t7799 * t352;
    let t7810 = t645 * t833;
    (t7789, t7793, t7795, t7796, t7797, t7800, t7803, t7810)
}
