//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1287/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1287<F: Float>(t128483: F, t128485: F, t128487: F, t128490: F, t128493: F, t128495: F, t128497: F, t128499: F, t128510: F, t128513: F, t129377: F, t27126: F, t28696: F, t28929: F, t33306: F, t7586: F, t7732: F, t8892: F) -> F {
    let t130984 = F::new(6.0) * t129377 * t28929 - F::new(2.0) * t27126 * t8892 - F::new(2.0) * t28696 * t7586 - F::new(2.0) * t33306 * t7732 - t128483 - t128485 - t128487 - t128490 - t128493 - t128495 - t128497 - t128499 - t128510 - t128513;
    t130984
}
