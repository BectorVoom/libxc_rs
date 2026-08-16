//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 789/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk789(t166: f64, t6880: f64, t2068: f64, t2271: f64, t2320: f64, t58: f64, t766: f64, t2330: f64, t2333: f64, t2332: f64, t287: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6881 = t6880 * t166;
    let t6885 = t2271 * t2068;
    let t6887 = t2320 * t58;
    let t6888 = t6887 * t766;
    let t6890 = t2330 * t2333;
    let t6897 = 1.0_f64 / t2332 / t287;
    (t6881, t6885, t6887, t6888, t6890, t6897)
}
