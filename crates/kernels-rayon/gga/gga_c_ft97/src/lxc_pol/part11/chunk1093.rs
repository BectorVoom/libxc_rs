//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1093/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1093(t8392: f64, t9794: f64, t9985: f64, t10074: f64, t10079: f64, t10080: f64, t10166: f64, t1901: f64, t1934: f64, t2409: f64, t242: f64, t2459: f64, t2599: f64, t2600: f64, t2606: f64, t41403: f64, t42819: f64, t42832: f64, t446: f64, t684: f64, t724: f64, t9787: f64, t9793: f64, t9983: f64) -> f64 {
    let t42834 = t8392 * t9794;
    let t42836 = t8392 * t9985;
    let t42850 = -4.0_f64 / 9.0_f64 * t446 * t724 * t10166 * t684 - 16.0_f64 / 27.0_f64 * t42819 - 8.0_f64 * t446 * t242 * t41403 - 8.0_f64 / 3.0_f64 * t1901 * t9787 * t9793 + 2.0_f64 / 3.0_f64 * t1901 * t2599 * t2600 * t1934 * t2459 + 8.0_f64 / 9.0_f64 * t42832 + 8.0_f64 / 9.0_f64 * t42834 - 4.0_f64 / 9.0_f64 * t42836 - 4.0_f64 / 3.0_f64 * t1901 * t2599 * t9983 * t2409 + 8.0_f64 / 3.0_f64 * t1901 * t10079 * t10080 * t2409 + 8.0_f64 / 3.0_f64 * t1901 * t2606 * t10074 * t2409;
    t42850
}
