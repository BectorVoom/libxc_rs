//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1260/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1260<F: Float>(t5633: F, t5748: F, t1464: F, t1497: F, t7202: F, t4135: F, t1395: F, t16623: F, t5677: F, t5671: F, t5756: F, t1468: F) -> (F, F, F, F, F) {
    let t21035 = t5748 * t5633;
    let t21036 = t1464 * t21035;
    let t21038 = t7202 * t1497;
    let t21039 = t4135 * t21038;
    let t21040 = t1395 * t21039;
    let t21041 = t1464 * t21040;
    let t21043 = t16623 * t5677;
    let t21044 = t1464 * t21043;
    let t21046 = t5756 * t5671;
    let t21047 = t1468 * t21046;
    (t21036, t21038, t21041, t21044, t21047)
}
