//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3112/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3112<F: Float>(t56176: F, t81439: F, t81442: F, t81445: F, t81448: F, t81451: F, t81454: F, t81457: F, t81460: F, t81463: F, t81466: F, t81469: F) -> F {
    let t81944 = F::new(0.11038e0) * t81439 - F::cast_from(0.8585111111111111111e-1_f64) * t81442 - F::new(0.27595e-1) * t81445 + F::new(0.49671e0) * t81448 + F::new(0.49671e0) * t81451 + F::new(0.149013e1) * t81454 + F::new(0.198684e1) * t81457 + F::new(0.16557e0) * t81460 - F::new(0.49671e0) * t81463 - F::new(0.99342e0) * t81466 + F::new(0.44152e0) * t81469 - F::cast_from(0.26837777777777777779e0_f64) * t56176;
    t81944
}
