//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1096/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1096(t10151: f64, t1882: f64, t259: f64, t41743: f64, t89: f64, t754: f64, t9802: f64, t10007: f64, t10014: f64, t10018: f64, t10051: f64, t10053: f64, t10092: f64, t13879: f64, t14098: f64, t1901: f64, t2373: f64, t2409: f64, t2574: f64, t2606: f64, t2619: f64, t3891: f64, t42404: f64, t42417: f64, t446: f64, t684: f64, t9787: f64, t9804: f64, t9808: f64, t9849: f64) -> f64 {
    let t42920 = t1882 * t10151;
    let t42928 = 280.0_f64 / 243.0_f64 * t89 * t41743 * t259;
    let t42939 = t9802 * t754;
    let t42960 = 4.0_f64 / 3.0_f64 * t42920 - 4.0_f64 / 3.0_f64 * t1901 * t10007 * t10018 * t684 + t42928 + 8.0_f64 / 3.0_f64 * t1901 * t3891 * t14098 * t42404 - 8.0_f64 / 9.0_f64 * t1901 * t13879 * t10014 + 4.0_f64 / 3.0_f64 * t1901 * t9787 * t9849 + 8.0_f64 / 9.0_f64 * t1901 * t42939 * t9804 + 8.0_f64 / 3.0_f64 * t1901 * t2606 * t10051 * t10053 * t684 + 8.0_f64 / 3.0_f64 * t1901 * t2606 * t9808 * t42417 - 4.0_f64 / 3.0_f64 * t1901 * t2606 * t10092 * t2409 + 4.0_f64 * t446 * t2574 * t2619 * t2373;
    t42960
}
