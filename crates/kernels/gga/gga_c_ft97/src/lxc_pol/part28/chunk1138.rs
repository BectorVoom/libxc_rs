//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1138/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1138<F: Float>(t139392: F, t1969: F, t5899: F, t925: F, t139312: F, t139321: F, t139324: F, t148381: F, t148385: F, t148388: F, t148392: F, t148396: F, t148401: F, t148405: F, t148410: F, t148414: F, t148419: F, t148422: F, t148426: F) -> (F, F) {
    let t148430 = t5899 * t1969 * t139392 * t925;
    let t148432 = -t148381 / F::new(8.0) - t148385 - t148388 / F::new(6.0) + F::new(2.0) / F::new(3.0) * t148392 + F::new(4.0) / F::new(3.0) * t148396 - t139312 / F::new(9.0) + t148401 / F::new(4.0) + t139321 - t139324 - F::new(8.0) / F::new(9.0) * t148405 + F::new(4.0) / F::new(9.0) * t148410 - F::new(4.0) / F::new(9.0) * t148414 + F::new(2.0) / F::new(27.0) * t148419 + F::new(4.0) / F::new(9.0) * t148422 - F::new(2.0) * t148426 + t148430 / F::new(18.0);
    (t148430, t148432)
}
