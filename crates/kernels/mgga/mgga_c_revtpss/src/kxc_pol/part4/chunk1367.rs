//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1367/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1367<F: Float>(t3678: F, t5323: F, t1235: F, t1238: F, t12800: F, t12976: F, t17280: F, t17283: F, t17290: F, t17296: F, t1791: F, t1808: F, t3644: F, t3663: F, t3667: F, t5320: F, t5327: F, t5391: F) -> F {
    let t17298 = F::new(0.15244095330869239812e-2) * t5323 * t3678;
    let t17299 = -F::new(0.21437009059034868486e-3) * t12976 * t1791 - F::new(0.42874018118069736972e-3) * t3667 * t5320 + F::new(0.15244095330869239812e-2) * t5391 * t3644 - F::new(0.14291339372689912324e-3) * t12800 * t1808 - F::new(0.21437009059034868486e-3) * t1235 * t17280 + F::new(0.22866142996303859718e-2) * t17283 * t1238 + F::new(0.11433071498151929859e-2) * t5323 * t3663 - F::new(0.42874018118069736972e-3) * t17290 * t1238 - F::new(0.21437009059034868486e-3) * t5327 * t3663 - t17296 + t17298;
    t17299
}
