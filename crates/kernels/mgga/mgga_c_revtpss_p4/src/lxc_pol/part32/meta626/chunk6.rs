//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1996/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1996<F: Float>(t60673: F, t7342: F, t101243: F, t101935: F, t101938: F, t108762: F, t108769: F, t108792: F, t108864: F, t2048: F, t26175: F, t28133: F, t28141: F, t28154: F, t28602: F, t28628: F, t29562: F, t30543: F, t6960: F, t6963: F, t7343: F, t7964: F, t95276: F) -> F {
    let t109926 = t60673 * t7342;
    let t109945 = F::new(20.0) / F::new(3.0) * t101243 * t28628 + F::new(20.0) / F::new(3.0) * t28154 * t101935 + F::new(20.0) / F::new(3.0) * t28154 * t101938 - F::new(5.0) / F::new(3.0) * t109926 * t6960 - F::new(2.0) / F::new(3.0) * t108769 * t2048 - F::new(10.0) / F::new(3.0) * t28602 * t28133 - F::new(4.0) / F::new(3.0) * t28141 * t7964 - F::new(5.0) / F::new(3.0) * t7343 * t108792 - F::new(2.0) / F::new(3.0) * t6963 * t30543 + F::new(10.0) * t95276 * t29562 + F::new(10.0) * t26175 * t108864 - F::new(2.0) / F::new(3.0) * t108762 * t2048;
    t109945
}
