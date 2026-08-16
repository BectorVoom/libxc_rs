//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2556/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2556<F: Float>(t15749: F, t3224: F, t3140: F, t4743: F, t3149: F, t3160: F, t1041: F, t1670: F, t42994: F, t11988: F, t4834: F, t15731: F, t3124: F) -> (F, F, F, F, F, F) {
    let t55154 = t3224 * t15749;
    let t55155 = F::cast_from(0.14291339372689912324e-3_f64) * t55154;
    let t55201 = t4743 * t3140;
    let t55202 = t55201 * t3149;
    let t55205 = t55201 * t3160;
    let t55247 = t1041 * t42994 * t1670;
    let t55272 = t4834 * t11988;
    let t55279 = t3124 * t15731;
    (t55155, t55202, t55205, t55247, t55272, t55279)
}
