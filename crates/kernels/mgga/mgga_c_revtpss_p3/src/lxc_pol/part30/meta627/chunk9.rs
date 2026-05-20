//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2183/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2183<F: Float>(t1940: F, t1963: F, t2403: F, t25206: F, t25215: F, t25436: F, t25440: F, t27376: F, t27382: F, t27391: F, t27402: F, t30: F, t4541: F, t7092: F, t7749: F, t7783: F, t92819: F, t98780: F, t98784: F, t98787: F, t98793: F, t99537: F, t99542: F, t99543: F, t99550: F, t99555: F, t99558: F) -> F {
    let t99563 = -F::new(3.0) * t92819 * t27376 + t27382 * t98780 - t98784 - F::new(3.0) * t27382 * t98787 + F::new(3.0) / F::new(2.0) * t2403 * t7783 * t25215 + F::new(3.0) * t4541 * t1963 * t98793 + t1940 * t99537 * t30 / F::new(2.0) + t99542 + F::new(3.0) * t2403 * t1963 * t99543 + F::new(3.0) / F::new(2.0) * t2403 * t25436 * t7749 + F::new(2.0) * t27382 * t99550 - t1940 * t25440 * t27391 - t1940 * t99555 * t7092 - F::new(3.0) * t25206 * t99558 - t1940 * t25440 * t27402;
    t99563
}
