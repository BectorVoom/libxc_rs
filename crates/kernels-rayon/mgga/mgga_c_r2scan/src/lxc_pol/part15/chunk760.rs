//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 760/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk760(t1577: f64, t6231: f64, t1593: f64, t1632: f64, t551: f64, t1592: f64, t2090: f64, t57: f64, t128: f64, t524: f64, t540: f64, t108: f64, t489: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6232 = t1577 * t6231;
    let t6235 = t551 * t1632 * t1593;
    let t6236 = t1592 * t6235;
    let t6238 = t2090 * t57;
    let t6239 = t6238 * t128;
    let t6240 = t524 * t6239;
    let t6241 = t6240 * t540;
    let t6243 = t489 * t108;
    (t6232, t6236, t6238, t6239, t6240, t6241, t6243)
}
