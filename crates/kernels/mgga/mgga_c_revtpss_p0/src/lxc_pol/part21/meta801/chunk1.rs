//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2908/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2908<F: Float>(t52664: F, t52677: F, t52690: F, t52702: F, t52716: F, t52729: F, t52743: F, t52756: F, t915: F, t935: F, t51973: F, t41361: F, t41363: F, t41369: F, t41549: F, t51849: F, t51853: F, t51858: F, t51863: F, t51867: F, t51871: F, t51875: F, t51961: F, t51965: F, t51967: F, t51971: F, t51978: F, t52028: F, t52031: F, t52033: F) -> (F, F) {
    let t52762 = F::new(1.0) * t915 * (t52664 + t52677 + t52690 + t52702 + t52716 + t52729 + t52743 + t52756) * t935;
    let t52774 = F::cast_from(0.23744444444444444444e-1_f64) * t51973;
    let t52782 = F::new(0.4274e0) * t51849 - F::cast_from(0.11872222222222222222e-1_f64) * t51853 - F::cast_from(0.52765432098765432099e-1_f64) * t51858 + F::new(0.10685e0) * t51863 + F::new(0.10685e0) * t51867 + F::cast_from(0.35616666666666666666e-1_f64) * t51871 - F::cast_from(0.42739999999999999999e0_f64) * t51875 + t41549 + F::cast_from(0.21369999999999999999e0_f64) * t51961 - F::cast_from(0.59361111111111111111e-1_f64) * t51965 + F::cast_from(0.17808333333333333333e-1_f64) * t51967 - F::cast_from(0.17808333333333333333e-1_f64) * t51971 - t52774 + F::cast_from(0.18467901234567901234e-1_f64) * t51978 + F::cast_from(0.55403703703703703702e-1_f64) * t41361 + F::cast_from(0.47488888888888888887e-1_f64) * t41363 - F::cast_from(0.23744444444444444444e-1_f64) * t41369 + F::new(0.2137e0) * t52028 + F::cast_from(0.23744444444444444444e0_f64) * t52031 + F::new(0.10685e0) * t52033;
    (t52762, t52782)
}
