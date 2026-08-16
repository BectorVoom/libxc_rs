//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 810/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk810(t1882: f64, t7626: f64, t7622: f64, t1901: f64, t34156: f64, t34158: f64, t34160: f64, t34164: f64, t34169: f64, t34174: f64, t34178: f64, t34183: f64, t34187: f64, t34191: f64, t446: f64) -> (f64, f64, f64) {
    let t34193 = 2.0_f64 / 9.0_f64 * t1882 * t7626;
    let t34195 = 2.0_f64 / 9.0_f64 * t1882 * t7622;
    let t34196 = t34156 - t34158 - 2.0_f64 / 9.0_f64 * t1901 * t34160 + 2.0_f64 / 3.0_f64 * t446 * t34164 - 2.0_f64 / 3.0_f64 * t446 * t34169 - 2.0_f64 * t446 * t34174 - 2.0_f64 * t446 * t34178 - 2.0_f64 / 3.0_f64 * t446 * t34183 + 4.0_f64 / 3.0_f64 * t446 * t34187 + t34191 + t34193 - t34195;
    (t34193, t34195, t34196)
}
