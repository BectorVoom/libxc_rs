//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1226/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1226<F: Float>(t102854: F, t127193: F, t127199: F, t127207: F, t127893: F, t127929: F, t127940: F, t127942: F, t127948: F, t1940: F, t26425: F, t27799: F, t27800: F, t27817: F, t28460: F, t32080: F, t32491: F, t33: F, t7432: F, t8677: F) -> F {
    let t128121 = -t127929 - t1940 * t7432 * t127207 / F::new(2.0) - t1940 * t102854 * t8677 / F::new(2.0) + F::new(3.0) * t26425 * t27799 * t127942 + t127940 * t27800 - t1940 * t28460 * t32080 / F::new(2.0) - t1940 * t32491 * t27817 / F::new(2.0) + t1940 * t127893 * t33 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t26425 * t127193 + t127948 - F::new(3.0) / F::new(2.0) * t26425 * t127199;
    t128121
}
