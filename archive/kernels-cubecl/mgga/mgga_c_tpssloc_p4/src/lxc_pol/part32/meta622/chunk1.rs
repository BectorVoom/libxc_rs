//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2029/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2029<F: Float>(t11832: F, t2127: F, t24684: F, t7324: F, t10401: F, t24739: F, t3610: F, t3624: F, t24740: F, t3604: F, t11791: F, t7345: F) -> (F, F, F, F, F, F) {
    let t86278 = F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t2127 * t11832;
    let t86292 = t7324 * t24684;
    let t86323 = t24739 * t10401;
    let t86324 = t3610 * t86323;
    let t86327 = t3624 * t86323;
    let t86330 = t3604 * t24740;
    let t86348 = t7345 * t11791;
    (t86278, t86292, t86324, t86327, t86330, t86348)
}
