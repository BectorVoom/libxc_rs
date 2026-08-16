//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1242/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1242<F: Float>(t2989: F, t3966: F, t2960: F, t4506: F, t10224: F, t1592: F, t973: F, t4528: F, t1599: F, t698: F, t135: F, t4542: F) -> (F, F, F, F, F, F) {
    let t13861 = t2989 * t3966;
    let t13893 = F::cast_from(0.49382716049382716048e-3_f64) * t2960 * t4506;
    let t13895 = t10224 * t1592;
    let t13896 = t973 * t13895;
    let t13907 = F::cast_from(0.14814814814814814814e-2_f64) * t2960 * t4528;
    let t13908 = t698 * t1599;
    let t13909 = t973 * t13908;
    let t13913 = t135 * t4542;
    (t13861, t13893, t13896, t13907, t13909, t13913)
}
