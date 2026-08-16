//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1040/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1040(t1959: f64, t31759: f64, t32426: f64, t32430: f64, t32434: f64, t32437: f64, t32438: f64, t32439: f64, t32441: f64, t32445: f64, t32450: f64, t32456: f64, t7073: f64, t7079: f64, t7083: f64, t8645: f64, t8649: f64, t8652: f64) -> f64 {
    let t32457 = 0.57119737665102352616e0_f64 * t32426 * t8652 + 0.57119737665102352616e0_f64 * t8649 * t32430 + 0.17347256376410398924e1_f64 * t32434 * t7073 - t32437 + t32438 - t32439 + 0.57119737665102352616e0_f64 * t8649 * t32441 - 0.17135921299530705785e1_f64 * t8649 * t32445 + 0.8673628188205199462e0_f64 * t32434 * t7079 - 0.8673628188205199462e0_f64 * t32450 * t1959 - 0.56468933516960933999e-3_f64 * t31759 - 0.8673628188205199462e0_f64 * t8645 * t7083 + t32456;
    t32457
}
