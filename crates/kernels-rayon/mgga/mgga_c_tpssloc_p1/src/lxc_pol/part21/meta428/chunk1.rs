//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1959/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1959(t15268: f64, t4934: f64, t1714: f64, t3469: f64, t460: f64, t1178: f64, t12606: f64, t1177: f64, t135: f64, t457: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15269 = t4934 * t15268;
    let t15273 = t1714 * t3469 * t460;
    let t15274 = t4934 * t15273;
    let t15277 = t1178 * t12606;
    let t15278 = t1177 * t15277;
    let t15281 = t135 * t457;
    (t15269, t15273, t15274, t15277, t15278, t15281)
}
