//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2078/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2078<F: Float>(t10475: F, t42342: F, t42345: F, t2770: F, t283: F, t11064: F, t42332: F, t11058: F, t1014: F, t42340: F, t42341: F, t3127: F) -> (F, F, F, F, F, F) {
    let t43385 = t42342 * t10475 * t42345;
    let t43398 = F::cast_from(1.0_f64) / t283 / t2770;
    let t43470 = t42332 * t11064;
    let t43473 = t42332 * t11058;
    let t43503 = t42340 * t42341 * t1014;
    let t43515 = t42340 * t42341 * t3127;
    (t43385, t43398, t43470, t43473, t43503, t43515)
}
