//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2056/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2056<F: Float>(t45972: F, t7565: F, t2121: F, t2247: F, t2251: F, t45963: F, t10309: F, t26754: F, t12627: F, t2142: F, t12640: F, t26982: F, t3565: F, t7635: F) -> (F, F, F, F, F, F, F) {
    let t96804 = t45972 * t7565;
    let t96810 = t2247 * t2251 * t2121;
    let t96824 = t45963 * t7565;
    let t96827 = t10309 * t26754;
    let t96861 = t12627 * t2142;
    let t96866 = t12640 * t2142;
    let t96870 = t26982 * t3565 * t7635;
    (t96804, t96810, t96824, t96827, t96861, t96866, t96870)
}
