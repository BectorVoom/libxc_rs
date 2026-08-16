//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 725/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk725(t7716: f64, t2197: f64, t7721: f64, t7724: f64, t7728: f64, t7733: f64, t7736: f64, t7768: f64, t7772: f64, t7775: f64, t7780: f64, t7786: f64, t7788: f64, t7791: f64, t7796: f64) -> (f64, f64) {
    let t7801 = 0.11607361111111111111e-2_f64 * t7716;
    let t7807 = -0.34752604166666666667e-3_f64 * t7768 * t2197 + 0.46377350260416666667e-4_f64 * t7772 * t7775 + 0.92673611111111111112e-3_f64 * t7780 * t2197 - t7786 - 0.11584201388888888889e-3_f64 * t7788 * t7791 + 0.34752604166666666667e-3_f64 * t7788 * t7796 + 0.34752604166666666667e-3_f64 * t7788 * t7775 + t7801 + 0.11607361111111111111e-2_f64 * t7721 + 0.17411041666666666666e-2_f64 * t7724 - 0.17411041666666666666e-2_f64 * t7728 - 0.46429444444444444443e-2_f64 * t7733 + 0.11607361111111111111e-2_f64 * t7736;
    (t7801, t7807)
}
