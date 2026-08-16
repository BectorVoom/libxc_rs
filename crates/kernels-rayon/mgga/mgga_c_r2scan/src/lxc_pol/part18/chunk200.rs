//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 200/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk200(t170: f64, t607: f64, t166: f64, t585: f64, t159: f64, t12: f64, t15: f64, t2: f64) -> (f64, f64, f64, f64, f64) {
    let t608 = t607 * t170;
    let t611 = t166 * t585;
    let t612 = t159 * t611;
    let t614 = 1.0_f64 / t15 / t12;
    let t615 = t614 * t2;
    (t608, t611, t612, t614, t615)
}
