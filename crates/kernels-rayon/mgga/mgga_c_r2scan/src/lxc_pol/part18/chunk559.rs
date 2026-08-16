//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 559/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk559(t3056: f64, t538: f64, t529: f64, t551: f64, t552: f64, t1569: f64, t3055: f64) -> (f64, f64, f64, f64) {
    let t3063 = t538 * t3056;
    let t3064 = t529 * t3063;
    let t3068 = t551 * t552 * t3056;
    let t3071 = t3055 * t1569;
    (t3063, t3064, t3068, t3071)
}
