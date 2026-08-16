//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 638/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk638(t3266: f64, t5630: f64, t26171: f64, t23339: f64, t3271: f64, t11810: f64, t6465: f64, t8372: f64, t1901: f64, t23148: f64, t26135: f64, t26139: f64, t26142: f64, t26147: f64, t26151: f64, t26156: f64, t26159: f64, t26163: f64, t26168: f64, t446: f64) -> (f64, f64, f64) {
    let t26172 = t5630 * t3266;
    let t26173 = t26171 * t26172;
    let t26176 = t23339 * t3271;
    let t26177 = t11810 * t26176;
    let t26180 = t8372 * t6465;
    let t26183 = -2.0_f64 / 9.0_f64 * t1901 * t26135 + t23148 / 27.0_f64 + t26139 / 9.0_f64 + t446 * t26142 / 3.0_f64 + t446 * t26147 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t26151 + t446 * t26156 / 3.0_f64 + t1901 * t26159 / 9.0_f64 - 2.0_f64 / 3.0_f64 * t1901 * t26163 - 2.0_f64 / 3.0_f64 * t1901 * t26168 - 2.0_f64 * t1901 * t26173 - 2.0_f64 / 3.0_f64 * t1901 * t26177 + t1901 * t26180 / 9.0_f64;
    (t26172, t26176, t26183)
}
