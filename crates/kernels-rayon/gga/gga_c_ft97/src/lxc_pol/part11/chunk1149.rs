//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1149/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1149(t10801: f64, t1882: f64, t10703: f64, t10712: f64, t10722: f64, t15312: f64, t1901: f64, t2409: f64, t2894: f64, t296: f64, t43332: f64, t44057: f64, t44131: f64, t44135: f64, t44145: f64, t44147: f64, t44149: f64, t446: f64, t684: f64, t835: f64) -> f64 {
    let t44151 = t1882 * t10801;
    let t44153 = -8.0_f64 / 9.0_f64 * t44057 + 4.0_f64 / 3.0_f64 * t446 * t835 * t2894 * t2409 + 2.0_f64 * t446 * t296 * t43332 - t446 * t296 * t44131 / 3.0_f64 + 8.0_f64 / 9.0_f64 * t44135 - 8.0_f64 / 3.0_f64 * t1901 * t15312 * t10712 * t684 - 4.0_f64 / 3.0_f64 * t1901 * t10703 * t10722 * t684 + 16.0_f64 / 9.0_f64 * t44145 + 8.0_f64 / 3.0_f64 * t44147 + 4.0_f64 / 3.0_f64 * t44149 + 4.0_f64 / 9.0_f64 * t44151;
    t44153
}
