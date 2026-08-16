//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1308/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1308(t2723: f64, t281: f64, t39675: f64, t39680: f64, t10523: f64, t10542: f64, t10960: f64, t2435: f64, t2482: f64, t39620: f64, t686: f64, t72: f64, t879: f64) -> (f64, f64, f64, f64) {
    let t39683 = t39680 * t281 * t39675 * t2723;
    let t39685 = t10542 * t10523;
    let t39687 = t2435 * t10960;
    let t39692 = t2482 * t879 * t72 * t686 * t39620;
    (t39683, t39685, t39687, t39692)
}
