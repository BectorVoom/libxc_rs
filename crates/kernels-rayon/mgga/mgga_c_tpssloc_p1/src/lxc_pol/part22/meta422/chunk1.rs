//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1737/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1737(t11668: f64, t19015: f64, t18232: f64, t3440: f64, t1017: f64, t6163: f64, t1210: f64, t1207: f64) -> (f64, f64, f64, f64) {
    let t19016 = t11668 * t19015;
    let t19019 = t3440 * t18232;
    let t19024 = t6163 * t1017;
    let t19025 = t1210 * t19024;
    let t19026 = t1207 * t19025;
    (t19016, t19019, t19025, t19026)
}
