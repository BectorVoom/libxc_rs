//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1094/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1094<F: Float>(t1940: F, t2071: F, t2257: F, t2403: F, t25198: F, t25208: F, t25211: F, t25215: F, t25446: F, t25449: F, t25452: F, t26425: F, t26581: F, t26585: F, t26590: F, t30: F, t4541: F, t605: F, t7010: F, t7092: F, t7428: F, t7432: F) -> F {
    let t26601 = F::new(3.0) * t4541 * t2071 * t25198 + F::new(3.0) * t2403 * t7428 * t7010 - F::new(3.0) * t26425 * t25208 + F::new(3.0) * t2403 * t2071 * t25211 + F::new(3.0) / F::new(2.0) * t2403 * t2071 * t25215 + t1940 * t26581 * t30 / F::new(2.0) - t1940 * t26585 * t7092 + t1940 * t7428 * t605 + t1940 * t26590 * t25446 - t1940 * t7432 * t25449 - t1940 * t7432 * t25452 / F::new(2.0) + t1940 * t2071 * t2257 / F::new(2.0);
    t26601
}
