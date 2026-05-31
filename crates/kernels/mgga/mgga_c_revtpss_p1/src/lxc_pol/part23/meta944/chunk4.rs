//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3102/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3102<F: Float>(t58114: F, t81439: F, t81442: F, t81445: F, t81448: F, t81451: F, t81454: F, t81457: F, t81460: F, t81463: F, t81466: F, t81469: F) -> F {
    let t81717 = F::cast_from(0.13892666666666666667e0_f64) * t81439 - F::cast_from(0.10805407407407407407e0_f64) * t81442 - F::cast_from(0.34731666666666666667e-1_f64) * t81445 + F::cast_from(0.62517e0_f64) * t81448 + F::cast_from(0.62517e0_f64) * t81451 + F::cast_from(0.187551e1_f64) * t81454 + F::cast_from(0.250068e1_f64) * t81457 + F::cast_from(0.20839e0_f64) * t81460 - F::cast_from(0.62517e0_f64) * t81463 - F::cast_from(0.125034e1_f64) * t81466 + F::cast_from(0.55570666666666666666e0_f64) * t81469 - t58114;
    t81717
}
