//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2576/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2576<F: Float>(t3717: F, t57659: F, t12865: F, t17400: F, t1222: F, t1781: F, t2438: F, t12854: F, t21013: F, t12808: F, t3698: F, t5047: F, t697: F) -> (F, F, F, F, F, F) {
    let t57660 = t3717 * t57659;
    let t57663 = t17400 * t12865;
    let t57687 = t1222 * t2438 * t1781;
    let t57707 = t12854 * t21013;
    let t57710 = t12808 * t21013;
    let t57726 = t1222 * t697 * t3698 * t5047;
    (t57660, t57663, t57687, t57707, t57710, t57726)
}
