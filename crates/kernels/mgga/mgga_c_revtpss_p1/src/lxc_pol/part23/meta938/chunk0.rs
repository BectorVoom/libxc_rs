//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3083/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3083<F: Float>(t1145: F, t141: F, t81207: F, t3417: F, t81169: F, t81173: F, t12254: F, t81165: F, t56176: F, t81439: F, t81442: F, t81445: F, t81448: F, t81451: F, t81454: F, t81457: F) -> (F, F, F, F, F) {
    let t81460 = t141 * t1145 * t81207;
    let t81463 = t141 * t3417 * t81169;
    let t81466 = t141 * t3417 * t81173;
    let t81469 = t141 * t12254 * t81165;
    let t81472 = F::cast_from(0.10954222222222222222e0_f64) * t81439 - F::cast_from(0.85199506172839506175e-1_f64) * t81442 - F::cast_from(0.27385555555555555556e-1_f64) * t81445 + F::cast_from(0.49293999999999999999e0_f64) * t81448 + F::cast_from(0.49293999999999999999e0_f64) * t81451 + F::new(0.147882e1) * t81454 + F::new(0.197176e1) * t81457 + F::cast_from(0.16431333333333333333e0_f64) * t81460 - F::cast_from(0.49293999999999999999e0_f64) * t81463 - F::cast_from(0.98587999999999999998e0_f64) * t81466 + F::cast_from(0.43816888888888888889e0_f64) * t81469 - F::cast_from(0.26574814814814814815e0_f64) * t56176;
    (t81460, t81463, t81466, t81469, t81472)
}
