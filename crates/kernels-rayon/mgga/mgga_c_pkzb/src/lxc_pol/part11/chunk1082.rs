//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1082/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1082(t2003: f64, t465: f64, t53: f64, t5633: f64, t2002: f64, t220: f64, t310: f64, t5999: f64, t5952: f64, t785: f64, t2021: f64, t296: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18199 = t465 * t2003;
    let t18204 = t53 * t5633;
    let t18210 = 1.0_f64 / t2002 / t220;
    let t18258 = 1.0_f64 / t5999 / t310;
    let t18278 = t5952 * t785;
    let t18290 = 1.0_f64 / t2021 / t296;
    (t18199, t18204, t18210, t18258, t18278, t18290)
}
