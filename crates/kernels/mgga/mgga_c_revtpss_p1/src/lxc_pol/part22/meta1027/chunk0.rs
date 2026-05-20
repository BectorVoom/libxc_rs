//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3599/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3599<F: Float>(t20340: F, t698: F, t20377: F, t5079: F, t3407: F, t43911: F, t56176: F, t56183: F, t56185: F, t68342: F, t68347: F, t68350: F, t68353: F, t68357: F, t68360: F, t68363: F, t68366: F) -> (F, F, F, F, F) {
    let t68368 = t698 * t20340;
    let t68370 = t698 * t20377;
    let t68372 = t5079 * t5079;
    let t68373 = t3407 * t68372;
    let t68379 = F::cast_from(0.33547222222222222222e0_f64) * t68342 + F::cast_from(0.40256666666666666666e1_f64) * t68347 - F::new(0.12077e1) * t68350 - F::new(0.72462e1) * t68353 - F::cast_from(0.40256666666666666666e0_f64) * t68357 + F::new(0.72462e1) * t68360 - F::cast_from(0.48307999999999999999e1_f64) * t68363 + F::cast_from(0.13418888888888888889e1_f64) * t68366 - F::new(0.22076e0) * t68368 - F::cast_from(0.49057777777777777778e-1_f64) * t68370 + F::new(0.16504875e0) * t68373 - F::cast_from(0.30661111111111111111e-1_f64) * t43911 - F::cast_from(0.35783703703703703705e0_f64) * t56176 + F::cast_from(0.10735111111111111112e1_f64) * t56183 - F::cast_from(0.80513333333333333336e0_f64) * t56185;
    (t68368, t68370, t68372, t68373, t68379)
}
