//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1096/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1096(t86: f64, t113: f64, t144337: f64, t144372: f64, t144411: f64, t144442: f64, t144569: f64, t144613: f64, t144647: f64, t144676: f64, t144703: f64, t144731: f64, t144763: f64, t145733: f64, t145769: f64, t146987: f64, t147004: f64, t147040: f64, t1577: f64, t18: f64, t32650: f64, t34791: f64, t5: f64, t505: f64, t7293: f64, t992: f64) -> f64 {
    let t87 = 10000000.0_f64 <= t86;
    let t147059 = piecewise3(t87, 0.0_f64, t5 * (t144337 + t144372 + t144411 + t144442 + t144569 + t144613 + t144647 + t144676 + t144703 + t144731 + t144763 + t145733 + t145769 + t146987 + t147004 + t147040) * t113 / 4.0_f64 + t5 * t34791 * t505 / 4.0_f64 + t5 * t32650 * t992 / 4.0_f64 - t5 * t7293 * t18 * t1577 / 2.0_f64);
    t147059
}
