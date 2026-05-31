//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1315/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1315<F: Float>(t6638: F, t92564: F, t19826: F, t7766: F, t19836: F, t92581: F, t29036: F, t33853: F, t10498: F, t1203: F, t33862: F, t5039: F, t96543: F) -> (F, F, F, F, F, F, F) {
    let t101713 = F::cast_from(2.0_f64) * t92564 * t6638;
    let t101716 = t19826 * t7766;
    let t101718 = F::cast_from(6.0_f64) * t92581 * t19836;
    let t101720 = F::cast_from(6.0_f64) * t33853 * t29036;
    let t101723 = F::cast_from(6.0_f64) * t10498 * t7766 * t6638;
    let t101730 = F::cast_from(24.0_f64) * t33862 * t29036 * t1203;
    let t101732 = F::cast_from(4.0_f64) * t96543 * t5039;
    (t101713, t101716, t101718, t101720, t101723, t101730, t101732)
}
