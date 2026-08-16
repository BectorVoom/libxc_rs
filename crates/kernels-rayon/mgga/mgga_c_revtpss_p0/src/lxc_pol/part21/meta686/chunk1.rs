//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2504/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2504(t12875: f64, t12916: f64, t5331: f64, t12871: f64, t5340: f64, t1222: f64, t12282: f64, t17471: f64, t1261: f64, t12944: f64, t3172: f64, t12932: f64, t3711: f64) -> (f64, f64, f64, f64, f64) {
    let t44773 = t5331 * t12916 * t12875;
    let t44776 = t5340 * t12916 * t12871;
    let t44786 = t1222 * t17471 * t12282;
    let t44789 = t1261 * t3172 * t12944;
    let t44792 = t3711 * t3172 * t12932;
    (t44773, t44776, t44786, t44789, t44792)
}
