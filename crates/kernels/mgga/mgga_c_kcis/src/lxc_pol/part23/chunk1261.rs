//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1261/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1261<F: Float>(t1490: F, t303: F, t98607: F, t2237: F, t27342: F, t28373: F, t3801: F, t3984: F, t7908: F, t8151: F, t94626: F, t98235: F, t98361: F, t98515: F, t98587: F, t98593: F, t98598: F, t98600: F, t98604: F) -> (F, F) {
    let t98609 = t303 * t98607 * t1490;
    let t98611 = -F::new(0.46336805555555555556e-3) * t7908 * t3984 * t28373 * t3801 + t98587 + F::new(0.27802083333333333334e-2) * t7908 * t98515 - F::new(0.92673611111111111112e-3) * t94626 * t98361 + F::new(0.66327777777777777776e-2) * t98593 + F::new(0.61782407407407407408e-3) * t94626 * t98235 - t98598 + F::new(0.69505208333333333333e-3) * t2237 * t98600 - t98604 + F::new(0.37069444444444444444e-2) * t8151 * t27342 - F::new(0.49745833333333333332e-2) * t98609;
    (t98609, t98611)
}
