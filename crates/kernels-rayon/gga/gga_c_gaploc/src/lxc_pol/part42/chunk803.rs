//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 803/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk803(t33294: f64, t9810: f64, t10628: f64, t549: f64, t6111: f64, t24505: f64, t2684: f64, t9438: f64, t3295: f64, t8802: f64, t9800: f64, t13052: f64, t1966: f64) -> (f64, f64, f64, f64, f64) {
    let t43681 = t33294 * t9810;
    let t43715 = t6111 * t549 * t10628;
    let t43718 = t2684 * t9438 * t24505;
    let t43756 = t9800 * t8802 * t3295;
    let t43758 = t1966 * t13052;
    (t43681, t43715, t43718, t43756, t43758)
}
