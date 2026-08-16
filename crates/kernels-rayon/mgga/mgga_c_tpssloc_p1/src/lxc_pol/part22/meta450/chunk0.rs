//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1808/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1808(t120: f64, t6347: f64, t1352: f64, t3805: f64, t5187: f64, t550: f64, t5249: f64, t1307: f64) -> (f64, f64, f64, f64) {
    let t19984 = t120 * t6347;
    let t19986 = t3805 * t19984 * t1352;
    let t19989 = t550 * t5187;
    let t19991 = t3805 * t5249 * t19989;
    let t19994 = t6347 * t1307;
    (t19986, t19989, t19991, t19994)
}
