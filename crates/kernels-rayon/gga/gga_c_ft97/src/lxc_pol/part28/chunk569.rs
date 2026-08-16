//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 569/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk569(t5857: f64, t8392: f64, t160: f64, t5842: f64, t1882: f64, t5882: f64, t91: f64, t9252: f64, t376: f64, t5890: f64, t5892: f64, t1369: f64, t5905: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23576 = t8392 * t5857;
    let t23581 = t160 * t5842;
    let t23598 = t1882 * t5882;
    let t23608 = t91 * t9252;
    let t23616 = t5890 * t376 * t5892;
    let t23629 = t1369 * t376 * t5905;
    (t23576, t23581, t23598, t23608, t23616, t23629)
}
