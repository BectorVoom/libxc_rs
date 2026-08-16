//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1109/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1109(t15899: f64, t8493: f64, t1983: f64, t1441: f64, t8319: f64, t510: f64, t1774: f64, t8320: f64, t7468: f64, t8526: f64, t12571: f64, t8301: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33082 = t8493 * t15899;
    let t33084 = 2.0_f64 * t1983 * t33082;
    let t33094 = t1441 * t8319;
    let t33096 = 2.0_f64 * t33094 * t510;
    let t33098 = 2.0_f64 * t8320 * t1774;
    let t33100 = 4.0_f64 * t8526 * t7468;
    let t33103 = t12571 * t8301;
    (t33082, t33084, t33094, t33096, t33098, t33100, t33103)
}
