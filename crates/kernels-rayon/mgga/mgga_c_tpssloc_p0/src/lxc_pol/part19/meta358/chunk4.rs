//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1303/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1303(t10633: f64, t2940: f64, t10629: f64, t2932: f64, t41827: f64, t959: f64, t10619: f64, t300: f64, t961: f64, t10957: f64, t3053: f64, t271: f64, t2770: f64) -> (f64, f64, f64, f64, f64) {
    let t42276 = 0.4101607543286562663e4_f64 * t2940 * t10633;
    let t42280 = 0.6233709278045326953e3_f64 * t959 * t10629 * t41827 * t2932;
    let t42281 = t300 * t10619;
    let t42283 = 0.23392894490538584828e1_f64 * t42281 * t961;
    let t42303 = t10957 * t3053;
    let t42308 = 1.0_f64 / t271 / t2770;
    (t42276, t42280, t42283, t42303, t42308)
}
