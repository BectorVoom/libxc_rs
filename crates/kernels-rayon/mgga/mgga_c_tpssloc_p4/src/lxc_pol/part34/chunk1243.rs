//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1243/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1243(t102192: f64, t102194: f64, t102215: f64, t102217: f64, t102219: f64, t102221: f64, t102248: f64, t106816: f64, t2032: f64, t26954: f64, t27979: f64, t7782: f64, t91996: f64, t96443: f64) -> f64 {
    let t108743 = -2.0_f64 * t106816 * t2032 - 2.0_f64 * t27979 * t7782 + 80.0_f64 / 3.0_f64 * t102192 + 40.0_f64 / 3.0_f64 * t102194 + 16.0_f64 / 3.0_f64 * t102215 + 32.0_f64 / 3.0_f64 * t102217 + 80.0_f64 / 3.0_f64 * t102219 + 32.0_f64 / 3.0_f64 * t102221 - 80.0_f64 * t102248 + 88.0_f64 / 9.0_f64 * t91996 + 20.0_f64 * t96443 * t26954;
    t108743
}
