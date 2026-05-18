//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1198/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1198<F: Float>(t198: F, t205: F, t8656: F, t2411: F, t34079: F, t102888: F, t106589: F, t121751: F, t1940: F, t2403: F, t26585: F, t27160: F, t27166: F, t27395: F, t27402: F, t28460: F, t28472: F, t32487: F, t32491: F, t32499: F, t32505: F, t32508: F, t34080: F, t34100: F, t605: F, t7092: F, t7749: F, t8657: F) -> (F, F, F) {
    let t127566 = t198 * t205 * t8656;
    let t127582 = t34079 * t2411;
    let t127592 = F::new(3.0) / F::new(2.0) * t2403 * t8657 * t27395 - F::new(3.0) / F::new(2.0) * t102888 * t32499 + F::new(3.0) * t127566 * t27160 + F::new(3.0) / F::new(2.0) * t2403 * t32487 * t7749 - t1940 * t28460 * t32508 / F::new(2.0) + t28472 * t106589 * t32505 - F::new(3.0) / F::new(2.0) * t121751 * t27166 - t1940 * t32491 * t27402 / F::new(2.0) - t1940 * t127582 * t7092 / F::new(2.0) + t1940 * t34080 * t605 / F::new(2.0) - t1940 * t26585 * t34100 / F::new(2.0);
    (t127566, t127582, t127592)
}
