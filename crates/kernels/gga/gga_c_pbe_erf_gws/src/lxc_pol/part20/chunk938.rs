//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 938/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk938<F: Float>(t2678: F, t34: F, t1714: F, t10353: F, t657: F, t10357: F, t1697: F, t3354: F, t422: F, t1642: F, t10514: F, t10517: F, t10519: F, t10521: F, t10525: F, t10536: F, t25: F, t2718: F, t5052: F, t5083: F, t7237: F, t7239: F, t7269: F, t7272: F) -> (F, F, F, F) {
    let t10539 = t2678 * t34;
    let t10540 = t1714 * t10539;
    let t10543 = t657 * t10353;
    let t10546 = t657 * t10357;
    let t10549 = t1697 * t3354;
    let t10550 = t10549 * t422;
    let t10551 = t657 * t10550;
    let t10554 = t1642 * t3354;
    let t10555 = t10554 * t422;
    let t10556 = t1714 * t10555;
    let t10559 = -F::cast_from(0.66666666666666666667e-2_f64) * t25 * t10514 + F::cast_from(0.14814814814814814815e-2_f64) * t10517 - F::cast_from(0.88888888888888888887e-2_f64) * t10519 + F::cast_from(0.44444444444444444445e-2_f64) * t10521 + F::cast_from(0.13333333333333333333e-1_f64) * t25 * t10525 - F::cast_from(0.17777777777777777778e-1_f64) * t7237 - F::cast_from(0.14814814814814814815e-1_f64) * t7239 - F::cast_from(0.74074074074074074073e-2_f64) * t5052 - F::cast_from(0.15996296296296296296e-1_f64) * t5083 - F::cast_from(0.31992592592592592592e-1_f64) * t7269 - F::cast_from(0.47988888888888888888e-1_f64) * t7272 - F::cast_from(0.29629629629629629629e-2_f64) * t25 * t10536 + F::cast_from(0.88888888888888888888e-2_f64) * t2718 * t10540 - F::cast_from(0.39999999999999999999e-1_f64) * t25 * t10543 - F::cast_from(0.53333333333333333332e-1_f64) * t2718 * t10546 + F::cast_from(0.13333333333333333333e-1_f64) * t25 * t10551 - F::cast_from(0.22222222222222222222e-2_f64) * t25 * t10556;
    (t10539, t10550, t10555, t10559)
}
