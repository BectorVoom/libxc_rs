//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2926/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2926<F: Float>(t15885: F, t993: F, t378: F, t51973: F, t41361: F, t41363: F, t41369: F, t42078: F, t51849: F, t51853: F, t51858: F, t51863: F, t51867: F, t51871: F, t51875: F, t51961: F, t51965: F, t51967: F, t51971: F, t51978: F, t52028: F, t52031: F, t52033: F) -> (F, F, F) {
    let t53222 = t15885 * t993;
    let t53223 = t53222 * t378;
    let t53243 = F::cast_from(0.19755555555555555556e-1_f64) * t51973;
    let t53251 = F::cast_from(0.35560000000000000001e0_f64) * t51849 - F::cast_from(0.9877777777777777778e-2_f64) * t51853 - F::cast_from(0.43901234567901234568e-1_f64) * t51858 + F::cast_from(0.88900000000000000002e-1_f64) * t51863 + F::cast_from(0.88900000000000000002e-1_f64) * t51867 + F::cast_from(0.29633333333333333334e-1_f64) * t51871 - F::cast_from(0.35560000000000000001e0_f64) * t51875 + t42078 + F::cast_from(0.17780000000000000001e0_f64) * t51961 - F::cast_from(0.49388888888888888889e-1_f64) * t51965 + F::cast_from(0.14816666666666666667e-1_f64) * t51967 - F::cast_from(0.14816666666666666667e-1_f64) * t51971 - t53243 + F::cast_from(0.15365432098765432099e-1_f64) * t51978 + F::cast_from(0.46096296296296296298e-1_f64) * t41361 + F::cast_from(0.39511111111111111113e-1_f64) * t41363 - F::cast_from(0.19755555555555555556e-1_f64) * t41369 + F::cast_from(0.1778e0_f64) * t52028 + F::cast_from(0.19755555555555555556e0_f64) * t52031 + F::cast_from(0.88900000000000000002e-1_f64) * t52033;
    (t53222, t53223, t53251)
}
