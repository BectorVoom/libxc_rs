//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1796/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1796<F: Float>(t831: F, t87261: F, t81808: F, t4191: F, t81749: F, t4240: F, t23069: F, t4159: F, t23062: F, t25106: F, t13176: F, t6613: F) -> (F, F, F, F, F, F, F) {
    let t87262 = t87261 * t831;
    let t87268 = F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t81808;
    let t87270 = t81749 * t4191;
    let t87272 = t81749 * t4240;
    let t87291 = t23069 * t4159;
    let t87293 = t23062 * t25106;
    let t87295 = t13176 * t6613;
    (t87262, t87268, t87270, t87272, t87291, t87293, t87295)
}
