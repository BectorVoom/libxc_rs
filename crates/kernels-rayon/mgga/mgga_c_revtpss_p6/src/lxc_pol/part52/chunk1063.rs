//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1063/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1063(t32198: f64, t32273: f64, t32280: f64, t32285: f64, t32293: f64, t32674: f64, t32678: f64, t32681: f64, t32682: f64, t32683: f64, t32686: f64, t32690: f64, t7298: f64, t7304: f64, t8706: f64) -> f64 {
    let t32698 = 0.7437465841810202164e-3_f64 * t32285 + 0.57119737665102352616e0_f64 * t8706 * t32674 + 0.57119737665102352616e0_f64 * t8706 * t32678 + t32681 + t32682 - t32683 - 0.3718732920905101082e-3_f64 * t32273 - 0.17135921299530705785e1_f64 * t8706 * t32686 + 0.8673628188205199462e0_f64 * t32690 * t7304 + 0.7437465841810202164e-3_f64 * t32280 + 0.14874931683620404328e-2_f64 * t32293 - 0.56468933516960933999e-3_f64 * t32198 + 0.17347256376410398924e1_f64 * t32690 * t7298;
    t32698
}
