//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 750/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk750(t6064: f64, t6086: f64, t6093: f64, t2086: f64, t776: f64, t23: f64, t271: f64) -> (f64, f64, f64) {
    let t6094 = t6086 * t6064;
    let t6095 = t6093 * t6094;
    let t6097 = t776 * t2086;
    let t6100 = 1.0_f64 / t23 / t271;
    (t6095, t6097, t6100)
}
