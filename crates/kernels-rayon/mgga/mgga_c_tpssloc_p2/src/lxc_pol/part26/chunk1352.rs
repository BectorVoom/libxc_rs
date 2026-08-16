//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1352/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1352(t1235: f64, t24594: f64, t24705: f64, t7327: f64, t1176: f64, t1184: f64, t24847: f64, t974: f64, t1009: f64, t460: f64, t27495: f64, t15702: f64, t7329: f64) -> (f64, f64, f64, f64) {
    let t85807 = t24594 * t1235;
    let t85814 = t24705 * t7327;
    let t85820 = t24847 * t974 * t1176 * t1184;
    let t85821 = t460 * t1009;
    let t85822 = t85821 * t27495;
    let t85824 = t85822 * t7329 * t15702;
    (t85807, t85814, t85820, t85824)
}
