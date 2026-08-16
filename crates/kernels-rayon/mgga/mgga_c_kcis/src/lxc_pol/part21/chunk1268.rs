//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1268/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1268(t3452: f64, t5026: f64, t1817: f64, t9568: f64, t8072: f64, t92415: f64, t1189: f64, t13106: f64, t14853: f64, t26930: f64, t14839: f64, t7754: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95432 = t5026 * t3452;
    let t95434 = t9568 * t1817;
    let t95436 = t92415 * t8072;
    let t95438 = t13106 * t1189;
    let t95440 = t26930 * t14853;
    let t95442 = t7754 * t14839;
    (t95432, t95434, t95436, t95438, t95440, t95442)
}
