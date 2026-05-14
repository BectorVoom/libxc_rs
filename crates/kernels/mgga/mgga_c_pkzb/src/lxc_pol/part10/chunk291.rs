//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 291/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk291<F: Float>(t178: F, t926: F, t404: F, t334: F, t344: F) -> (F, F, F, F) {
    let t927 = t178 * t926;
    let t929 = 0.14291339372689912324e-3 * t404 * t927;
    let t930 = t344 * t334;
    let t931 = 1.0 / t930;
    (t927, t929, t930, t931)
}
