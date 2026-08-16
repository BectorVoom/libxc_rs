//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 782/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk782(t538: f64, t7605: f64, t6155: f64, t1634: f64, t2651: f64, t252: f64, t5094: f64, t146: f64) -> (f64, f64, f64, f64, f64) {
    let t7606 = t538 * t7605;
    let t7608 = 0.10975748638225852664e-1_f64 * t6155 * t7606;
    let t7610 = 0.23115257973478049502e0_f64 * t2651 * t1634;
    let t7613 = t5094 * t252;
    let t7614 = t146 * t7613;
    (t7606, t7608, t7610, t7613, t7614)
}
