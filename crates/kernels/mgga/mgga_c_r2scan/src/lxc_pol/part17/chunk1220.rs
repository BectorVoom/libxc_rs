//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1220/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1220<F: Float>(t38452: F, t39429: F, t39464: F, t39470: F, t39482: F, t41384: F, t41385: F, t41386: F, t41387: F, t41392: F, t43057: F, t43061: F) -> F {
    let t44209 = F::new(0.10975748638225852664e0) * t43057 + F::new(0.62295486109113302474e-1) * t39429 + t41384 - t41385 - t41386 + t41387 - t41392 - F::new(0.23804984598836975487e0) * t39464 - F::new(0.57829097596741960691e-3) * t39470 + F::new(0.87327386630866483588e-2) * t43061 - t38452 + F::new(0.62295486109113302474e-1) * t39482;
    t44209
}
