//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2578/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2578<F: Float>(t56176: F, t56183: F, t56228: F, t2439: F, t5101: F, t1729: F, t9303: F, t5095: F, t5098: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t58073 = F::new(8.0) / F::new(27.0) * t56176;
    let t58075 = F::new(8.0) / F::new(9.0) * t56183;
    let t58090 = F::new(4.0) / F::new(9.0) * t56228;
    let t58114 = F::cast_from(0.45908888888888888888e0_f64) * t56176;
    let t58117 = F::cast_from(0.13772666666666666666e1_f64) * t56183;
    let t58134 = F::cast_from(0.68863333333333333332e0_f64) * t56228;
    let t58145 = t2439 * t5101;
    let t58146 = F::cast_from(0.34731666666666666667e0_f64) * t58145;
    let t58153 = t9303 * t1729;
    let t58165 = t2439 * t5095;
    let t58166 = F::cast_from(0.11577222222222222222e0_f64) * t58165;
    let t58225 = t2439 * t5098;
    (t58073, t58075, t58090, t58114, t58117, t58134, t58145, t58146, t58153, t58165, t58166, t58225)
}
