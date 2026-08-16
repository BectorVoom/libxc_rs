//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1031/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1031(t146: f64, t6533: f64, t774: f64, t110: f64, t252: f64, t6359: f64, t545: f64, t7613: f64, t19790: f64, t495: f64, t1559: f64, t283: f64) -> (f64, f64, f64, f64, f64) {
    let t22796 = t146 * t6533 * t774;
    let t22820 = t146 * t110 * t6359 * t252;
    let t22868 = t545 * t7613;
    let t22948 = t19790 * t495;
    let t23038 = t1559 * t1559;
    let t23040 = 1.0_f64 / t283 / t23038;
    (t22796, t22820, t22868, t22948, t23040)
}
