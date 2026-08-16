//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1142/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1142<F: Float>(t10309: F, t121646: F, t119457: F, t1925: F, t32589: F, t8442: F, t45963: F, t8619: F, t32597: F, t32602: F, t2411: F, t32486: F) -> (F, F, F, F, F, F, F) {
    let t121647 = t10309 * t121646;
    let t121656 = t119457 * t1925;
    let t121660 = t10309 * t32589;
    let t121661 = t8442 * t1925;
    let t121665 = t45963 * t8619;
    let t121689 = t32597 * t32602;
    let t121716 = t32486 * t2411;
    (t121647, t121656, t121660, t121661, t121665, t121689, t121716)
}
