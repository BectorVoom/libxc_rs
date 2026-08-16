//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 982/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk982(t1495: f64, t210: f64, t5544: f64, t10026: f64, t10029: f64, t13368: f64, t16942: f64, t16954: f64, t16988: f64, t16990: f64, t16993: f64, t16995: f64, t17000: f64, t2571: f64) -> (f64, f64) {
    let t21008 = t210 * t1495 * t5544;
    let t21011 = 7.0_f64 / 1536.0_f64 * t16942 + 7.0_f64 / 384.0_f64 * t16954 - 35.0_f64 / 384.0_f64 * t16988 + 7.0_f64 / 192.0_f64 * t16990 - t10026 - 7.0_f64 / 16.0_f64 * t16993 + 7.0_f64 / 48.0_f64 * t16995 - 7.0_f64 / 1536.0_f64 * t17000 - t10029 - 119.0_f64 / 1152.0_f64 * t13368 + 3.0_f64 / 16.0_f64 * t2571 * t21008;
    (t21008, t21011)
}
