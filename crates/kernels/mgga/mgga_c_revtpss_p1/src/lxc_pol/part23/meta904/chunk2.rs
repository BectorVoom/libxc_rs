//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2901/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2901<F: Float>(t41361: F, t41520: F, t51978: F, t52337: F, t63276: F, t63278: F, t77499: F, t77503: F, t77505: F, t77507: F, t77509: F, t77515: F, t77518: F, t77521: F, t77527: F, t77531: F, t77535: F, t77539: F, t77543: F, t77547: F) -> F {
    let t77549 = F::cast_from(0.34336419753086419753e-2_f64) * t77499 - F::cast_from(0.92708333333333333333e-2_f64) * t77503 + F::cast_from(0.30902777777777777778e-2_f64) * t77505 - F::cast_from(0.12361111111111111111e-1_f64) * t77507 + F::cast_from(0.18541666666666666667e-1_f64) * t77509 - F::cast_from(0.18541666666666666667e-1_f64) * t63276 + F::cast_from(0.61805555555555555556e-2_f64) * t63278 + t41520 + F::new(0.11125e0) * t77515 - F::cast_from(0.30902777777777777777e-1_f64) * t77518 - F::new(0.166875e0) * t77521 - t52337 + F::cast_from(0.28842592592592592592e-1_f64) * t51978 + F::cast_from(0.96141975308641975307e-2_f64) * t41361 - F::cast_from(0.18541666666666666666e-1_f64) * t77527 - F::cast_from(0.18541666666666666666e-1_f64) * t77531 + F::new(0.2225e0) * t77535 - F::new(0.166875e0) * t77539 + F::cast_from(0.55625000000000000001e-1_f64) * t77543 + F::cast_from(0.55625000000000000001e-1_f64) * t77547;
    t77549
}
