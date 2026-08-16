//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1239/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1239<F: Float>(t3232: F, t4176: F, t3270: F, t3269: F, t10663: F, t12422: F, t12384: F, t37271: F, t11626: F, t40713: F, t11540: F, t40276: F) -> (F, F, F, F, F) {
    let t43775 = t4176 * t3232;
    let t43776 = t3270 * t43775;
    let t43778 = t3269 * t43776 / F::cast_from(4.0_f64);
    let t43780 = t12422 * t10663 / F::cast_from(4.0_f64);
    let t43782 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t37271 * t12384;
    let t43783 = t40713 * t11626;
    let t43785 = t40276 * t11540 / F::cast_from(2.0_f64);
    (t43778, t43780, t43782, t43783, t43785)
}
