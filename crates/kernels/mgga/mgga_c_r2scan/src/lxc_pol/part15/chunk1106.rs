//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1106/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1106<F: Float>(t11856: F, t3270: F, t3269: F, t10940: F, t11545: F, t1234: F, t3574: F, t10610: F, t3263: F, t10619: F, t11523: F, t2259: F, t2867: F) -> (F, F, F, F, F) {
    let t39274 = t3270 * t11856;
    let t39276 = t3269 * t39274 / F::new(2.0);
    let t39278 = F::new(5.0) / F::new(16.0) * t10940 * t11545;
    let t39279 = t3574 * t1234;
    let t39282 = F::new(3.0) / F::new(2.0) * t10610 * t3263 * t39279;
    let t39284 = t11523 * t10619 / F::new(2.0);
    let t39286 = t2867 * t2259;
    (t39276, t39278, t39282, t39284, t39286)
}
