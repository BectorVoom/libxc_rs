//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 675/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk675(t114: f64, t5086: f64, t133: f64, t1541: f64, t146: f64, t1603: f64, t2228: f64, t2158: f64, t147: f64, t2182: f64, t1591: f64, t2132: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5087 = t114 * t5086;
    let t5094 = t1541 * t133;
    let t5095 = t146 * t5094;
    let t5100 = t2228 * t1603;
    let t5101 = t5100 * t2158;
    let t5103 = t2182 * t147;
    let t5108 = t1591 * t2132;
    (t5087, t5094, t5095, t5100, t5101, t5103, t5108)
}
