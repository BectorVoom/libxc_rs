//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 428/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk428<F: Float>(t424: F, t725: F, t41: F, t410: F, t661: F, t1473: F, t1474: F, t1475: F, t1476: F, t1714: F, t1717: F) -> (F, F, F, F) {
    let t1793 = t424 * t725;
    let t1794 = t41 * t1793;
    let t1796 = t410 * t661;
    let t1800 = -F::cast_from(0.21099166666666666667e0_f64) * t1714 + F::cast_from(0.16879333333333333333e1_f64) * t1717 + t1473 + t1474 + t1475 + t1476;
    (t1793, t1794, t1796, t1800)
}
