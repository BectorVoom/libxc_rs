//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 987/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk987<F: Float>(t334: F, t369: F, t86: F, t1143: F, t245: F, t238: F, t3419: F, t3393: F, t3416: F, t3402: F, t1157: F, t752: F) -> (F, F, F, F, F, F) {
    let t10541 = F::new(0.11791604938271604938e-1) * t86 * t334 * t369;
    let t10544 = t1143 * t245;
    let t10548 = t86 * t238 * t3419;
    let t10552 = t3393 * t3416;
    let t10554 = t3393 * t3402;
    let t10556 = t752 * t1157;
    (t10541, t10544, t10548, t10552, t10554, t10556)
}
