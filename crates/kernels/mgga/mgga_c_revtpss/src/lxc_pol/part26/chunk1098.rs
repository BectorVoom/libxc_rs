//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1098/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1098<F: Float>(t10416: F, t1312: F, t13435: F, t13440: F, t2055: F, t2322: F, t2371: F, t26153: F, t26210: F, t26399: F, t26676: F, t5523: F, t670: F, t7359: F, t7373: F) -> F {
    let t26699 = F::new(2.0) * t10416 * t2055 + F::new(2.0) * t1312 * t26153 + F::new(4.0) * t13435 * t2055 + F::new(2.0) * t13440 * t2055 + F::new(4.0) * t2322 * t7373 + F::new(2.0) * t2371 * t7359 + F::new(4.0) * t26399 * t670 + F::new(4.0) * t5523 * t7373 + t26210 + F::new(2.0) * t26676;
    t26699
}
