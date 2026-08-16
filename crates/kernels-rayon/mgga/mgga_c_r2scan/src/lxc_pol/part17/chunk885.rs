//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 885/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk885(t2148: f64, t9445: f64, t2147: f64, t1632: f64, t3216: f64, t551: f64, t549: f64, t2670: f64, t2731: f64, t133: f64, t255: f64, t3177: f64) -> (f64, f64, f64, f64, f64) {
    let t9446 = t2148 * t9445;
    let t9447 = t2147 * t9446;
    let t9451 = t1632 * t3216;
    let t9452 = t551 * t9451;
    let t9453 = t549 * t9452;
    let t9458 = t2670 * t2731;
    let t9463 = t133 * t3177 * t255;
    (t9447, t9451, t9453, t9458, t9463)
}
