//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1310/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1310(t1662: f64, t93426: f64, t93427: f64, t1009: f64, t14400: f64, t14633: f64, t3200: f64, t92808: f64, t8048: f64, t9562: f64, t1014: f64, t27925: f64) -> (f64, f64, f64, f64, f64) {
    let t96105 = t93426 * t1662 * t93427;
    let t96108 = t14400 * t1009;
    let t96116 = t3200 * t92808 * t14633;
    let t96121 = t9562 * t8048;
    let t96123 = t1014 * t27925;
    (t96105, t96108, t96116, t96121, t96123)
}
