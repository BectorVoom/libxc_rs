//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 917/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk917<F: Float>(t10628: F, t1234: F, t797: F, t3262: F, t3263: F, t3264: F, t792: F, t3276: F, t3424: F, t885: F, t1108: F, t1353: F) -> (F, F, F, F, F, F, F, F) {
    let t10629 = t10628 / F::cast_from(2.0_f64);
    let t10630 = t797 * t1234;
    let t10632 = t3262 * t3263 * t10630;
    let t10633 = F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t10632;
    let t10634 = t3264 * t792;
    let t10635 = t3276 * t10634;
    let t10636 = t3262 * t10635;
    let t10637 = F::cast_from(15.0_f64) / F::cast_from(8.0_f64) * t10636;
    let t10638 = t3424 * t885;
    let t10639 = F::cast_from(2.0_f64) * t10638;
    let t10640 = t1353 * t1108;
    (t10629, t10630, t10633, t10634, t10635, t10637, t10639, t10640)
}
