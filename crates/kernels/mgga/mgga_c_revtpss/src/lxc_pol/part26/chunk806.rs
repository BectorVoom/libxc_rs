//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 806/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk806<F: Float>(t10345: F, t10357: F, t10361: F, t10364: F, t10369: F, t10373: F, t10376: F, t10379: F, t2270: F, t2276: F, t2279: F, t44: F, t49: F, t56: F, t614: F, t617: F) -> F {
    let t10380 = -F::new(1232.0) / F::new(27.0) * t10345 * t49 + F::new(220.0) / F::new(9.0) * t2270 * t617 - F::new(20.0) / F::new(9.0) * t614 * t2276 - F::new(20.0) / F::new(3.0) * t614 * t2279 - F::new(5.0) / F::new(108.0) * t44 * t10357 + F::new(5.0) / F::new(6.0) * t44 * t10361 + F::new(5.0) / F::new(6.0) * t44 * t10364 + F::new(5.0) / F::new(108.0) * t56 * t10369 + F::new(5.0) / F::new(6.0) * t56 * t10373 - F::new(5.0) / F::new(6.0) * t56 * t10376 + t10379;
    t10380
}
