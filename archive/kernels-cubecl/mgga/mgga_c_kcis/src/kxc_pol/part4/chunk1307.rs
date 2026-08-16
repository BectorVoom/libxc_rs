//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1307/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1307<F: Float>(t12274: F, t2013: F, t3738: F, t5757: F, t1464: F, t3722: F, t5756: F, t1395: F, t11776: F, t2012: F, t3728: F, t5761: F) -> (F, F, F, F, F) {
    let t16756 = t12274 * t2013;
    let t16758 = t3738 * t5757;
    let t16759 = t1464 * t16758;
    let t16761 = t5756 * t3722;
    let t16762 = t1395 * t16761;
    let t16763 = t1464 * t16762;
    let t16765 = t11776 * t2012;
    let t16766 = t1464 * t16765;
    let t16768 = t3728 * t5761;
    (t16756, t16759, t16763, t16766, t16768)
}
