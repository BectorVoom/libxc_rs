//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 345/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk345<F: Float>(t1306: F, t1309: F, t469: F, t465: F) -> (F, F, F, F) {
    let t1311 = -t1306 - F::new(0.17808333333333333333e-1) * t1309;
    let t1313 = F::new(0.62182e-1) * t1311 * t469;
    let t1314 = t465 * t465;
    let t1315 = F::new(1.0) / t1314;
    (t1311, t1313, t1314, t1315)
}
