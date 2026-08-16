//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 811/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk811(t108: f64, t7151: f64, t7152: f64, t7154: f64, t7162: f64, t486: f64, t95: f64, t5052: f64, t910: f64, t1543: f64, t1541: f64, t2526: f64) -> (f64, f64, f64, f64) {
    let t7165 = (t7151 + t7152 + t7154 + t7162) * t108;
    let t7175 = t486 * t95;
    let t7180 = t5052 * t910;
    let t7181 = t7180 * t1543;
    let t7184 = t1541 * t2526;
    (t7165, t7175, t7181, t7184)
}
