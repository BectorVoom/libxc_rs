//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 701/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk701<F: Float>(t1923: F, t207: F, t1937: F, t4741: F, t5246: F, t5416: F, t5418: F, t5422: F, t5424: F, t5426: F, t1691: F, t1986: F) -> (F, F, F) {
    let t5512 = t207 * t1923;
    let t5513 = t1937 * t5512;
    let t5523 = F::new(0.235315e2) * t5246 - F::cast_from(0.94126000000000000001e1_f64) * t5416 + F::cast_from(0.14641822222222222222e2_f64) * t5418 - F::new(0.16831e1) * t5422 + F::cast_from(0.11220666666666666667e1_f64) * t5424 - F::cast_from(0.13090777777777777778e1_f64) * t5426 - F::cast_from(0.32416222222222222223e0_f64) * t4741;
    let t5524 = t5523 * t207;
    let t5527 = t1986 * t1691;
    (t5513, t5524, t5527)
}
