//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1315/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1315(t6638: f64, t92564: f64, t19826: f64, t7766: f64, t19836: f64, t92581: f64, t29036: f64, t33853: f64, t10498: f64, t1203: f64, t33862: f64, t5039: f64, t96543: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t101713 = 2.0_f64 * t92564 * t6638;
    let t101716 = t19826 * t7766;
    let t101718 = 6.0_f64 * t92581 * t19836;
    let t101720 = 6.0_f64 * t33853 * t29036;
    let t101723 = 6.0_f64 * t10498 * t7766 * t6638;
    let t101730 = 24.0_f64 * t33862 * t29036 * t1203;
    let t101732 = 4.0_f64 * t96543 * t5039;
    (t101713, t101716, t101718, t101720, t101723, t101730, t101732)
}
