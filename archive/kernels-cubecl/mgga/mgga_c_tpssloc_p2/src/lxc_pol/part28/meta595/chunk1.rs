//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1892/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1892<F: Float>(t1649: F, t2749: F, t23788: F, t57893: F, t2752: F, t13487: F, t1390: F, t16018: F, t26062: F, t645: F, t72: F, t26066: F) -> (F, F, F, F, F, F) {
    let t89982 = t1649 * t2749;
    let t89987 = t23788 * t57893;
    let t89992 = t2752 * t1649;
    let t89993 = t89992 * t13487;
    let t90023 = t1390 * t16018;
    let t90072 = t72 * t26062 * t645;
    let t90076 = t72 * t26066 * t645;
    (t89982, t89987, t89993, t90023, t90072, t90076)
}
