//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2518/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2518<F: Float>(t2475: F, t808: F, t11028: F, t1580: F, t2439: F, t10504: F, t15002: F, t9285: F, t10505: F, t137: F, t41011: F, t11015: F, t4325: F) -> (F, F, F, F, F) {
    let t51176 = t808 * t2475;
    let t51199 = t2439 * t11028 * t1580;
    let t51203 = t10504 * t15002 * t9285;
    let t51207 = t41011 * t15002 * t137 * t10505;
    let t51208 = F::cast_from(0.69394917116090352834e-2_f64) * t51207;
    let t51211 = t4325 * t11015;
    (t51176, t51199, t51203, t51208, t51211)
}
