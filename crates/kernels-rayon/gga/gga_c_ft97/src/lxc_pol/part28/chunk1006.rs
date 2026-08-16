//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1006/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1006(t101975: f64, t1286: f64, t136000: f64, t136098: f64, t137488: f64, t1564: f64, t1642: f64, t22907: f64, t25523: f64, t25569: f64, t25605: f64, t25611: f64, t25617: f64, t26128: f64, t28: f64, t32355: f64, t34358: f64, t34362: f64, t378: f64, t5495: f64, t5501: f64, t5507: f64, t7166: f64, t7212: f64, t925: f64) -> f64 {
    let t144613 = -t5501 * t1564 * t136000 * t925 / 18.0_f64 - 2.0_f64 / 3.0_f64 * t1286 * t28 * t5507 * t101975 + t136098 / 9.0_f64 - t1286 * t28 * t32355 * t25523 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t5495 * t34362 - t1286 * t28 * t32355 * t26128 / 3.0_f64 - t5495 * t34358 / 3.0_f64 + t5501 * t378 * t7212 * t25611 / 9.0_f64 - t5501 * t1642 * t7212 * t25617 / 27.0_f64 - t5501 * t137488 * t25605 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t5501 * t22907 * t25569 - 2.0_f64 / 9.0_f64 * t5501 * t378 * t7166 * t25611 + 2.0_f64 / 27.0_f64 * t5501 * t1642 * t7166 * t25617;
    t144613
}
