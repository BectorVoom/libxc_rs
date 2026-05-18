//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 704/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk704<F: Float>(t224: F, t718: F, t1981: F, t1719: F, t1821: F, t695: F, t1945: F, t4741: F, t5246: F, t5416: F, t5418: F, t5422: F, t5424: F, t5426: F) -> (F, F, F, F, F) {
    let t5564 = t718 * t224;
    let t5567 = t1981 * t224;
    let t5568 = t1821 * t1719;
    let t5569 = t5568 * t695;
    let t5572 = t1945 * t224;
    let t5582 = F::new(0.126595e2) * t5246 - F::new(0.50638000000000000001e1) * t5416 + F::new(0.78770222222222222223e1) * t5418 - F::new(0.81910000000000000002e0) * t5422 + F::new(0.54606666666666666667e0) * t5424 - F::new(0.63707777777777777777e0) * t5426 - F::new(0.25559851851851851851e0) * t4741;
    (t5564, t5567, t5569, t5572, t5582)
}
