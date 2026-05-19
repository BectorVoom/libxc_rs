//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1202/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1202<F: Float>(t11531: F, t481: F, t3262: F, t3276: F, t3618: F, t3270: F, t10667: F, t10680: F, t11587: F, t37501: F, t10673: F, t11591: F, t37505: F) -> (F, F, F, F) {
    let t40416 = t11531 * t481;
    let t40419 = F::new(15.0) / F::new(8.0) * t3262 * t3276 * t40416;
    let t40420 = t3618 * t481;
    let t40421 = t3270 * t40420;
    let t40423 = F::new(3.0) / F::new(2.0) * t10667 * t40421;
    let t40425 = t10680 * t11587 * t37501;
    let t40426 = F::cast_from(0.72042316457491791906e-3_f64) * t40425;
    let t40428 = t10673 * t11591 * t37505;
    (t40419, t40423, t40426, t40428)
}
