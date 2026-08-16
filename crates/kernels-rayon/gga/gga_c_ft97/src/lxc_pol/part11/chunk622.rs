//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 622/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk622(t86: f64, t112: f64, t113: f64, t1927: f64, t1934: f64, t5: f64, t502: f64, t505: f64, t8598: f64, t8608: f64, t2235: f64, t177: f64, t2280: f64) -> (f64, f64, f64) {
    let t87 = 10000000.0_f64 <= t86;
    let t8613 = piecewise3(t87, 0.0_f64, t5 * t8598 * t113 / 4.0_f64 + 3.0_f64 / 4.0_f64 * t5 * t1927 * t505 + 3.0_f64 / 4.0_f64 * t5 * t502 * t1934 + t5 * t112 * t8608 / 4.0_f64);
    let t8614 = t5 * t2235;
    let t8618 = 1.0_f64 / t2280 / t177;
    (t8613, t8614, t8618)
}
