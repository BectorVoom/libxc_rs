//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 725/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk725<F: Float>(t7716: F, t2197: F, t7721: F, t7724: F, t7728: F, t7733: F, t7736: F, t7768: F, t7772: F, t7775: F, t7780: F, t7786: F, t7788: F, t7791: F, t7796: F) -> (F, F) {
    let t7801 = F::cast_from(0.11607361111111111111e-2_f64) * t7716;
    let t7807 = -F::cast_from(0.34752604166666666667e-3_f64) * t7768 * t2197 + F::cast_from(0.46377350260416666667e-4_f64) * t7772 * t7775 + F::cast_from(0.92673611111111111112e-3_f64) * t7780 * t2197 - t7786 - F::cast_from(0.11584201388888888889e-3_f64) * t7788 * t7791 + F::cast_from(0.34752604166666666667e-3_f64) * t7788 * t7796 + F::cast_from(0.34752604166666666667e-3_f64) * t7788 * t7775 + t7801 + F::cast_from(0.11607361111111111111e-2_f64) * t7721 + F::cast_from(0.17411041666666666666e-2_f64) * t7724 - F::cast_from(0.17411041666666666666e-2_f64) * t7728 - F::cast_from(0.46429444444444444443e-2_f64) * t7733 + F::cast_from(0.11607361111111111111e-2_f64) * t7736;
    (t7801, t7807)
}
