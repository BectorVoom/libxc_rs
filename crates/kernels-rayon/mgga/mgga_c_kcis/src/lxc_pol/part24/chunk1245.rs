//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1245/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1245(t1262: f64, t6272: f64, t1267: f64, t5310: f64, t92651: f64, t11081: f64, t26960: f64, t29116: f64, t1092: f64, t1121: f64, t27763: f64, t6696: f64) -> (f64, f64, f64, f64, f64) {
    let t100314 = t6272 * t1262;
    let t100319 = t6272 * t1267;
    let t100321 = t5310 * t92651 * t100319;
    let t100330 = t26960 * t11081 * t29116;
    let t100340 = t1092 * t27763 * t6696 * t1121;
    (t100314, t100319, t100321, t100330, t100340)
}
