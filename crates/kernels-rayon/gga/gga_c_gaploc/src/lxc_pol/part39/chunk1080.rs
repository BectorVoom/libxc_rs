//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1080/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1080(t46952: f64, t493: f64, t105: f64, t492: f64, t42687: f64, t42689: f64, t42691: f64, t42694: f64, t42695: f64, t42698: f64, t42700: f64, t42703: f64, t42706: f64, t42708: f64) -> (f64, f64) {
    let t46953 = t493 * t46952;
    let t46956 = 0.28455006635676149599e-1_f64 * t105 * t492 * t46953;
    let t46959 = -t46956 + t42687 - t42689 - t42691 - t42694 - 0.1138200265427045984e0_f64 * t42695 + t42698 - 0.85365019907028448797e-1_f64 * t42700 + t42703 + t42706 + t42708;
    (t46953, t46959)
}
