//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1003/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1003<F: Float>(t14776: F, t14791: F, t1537: F, t1527: F, t4459: F, t507: F, t4462: F, t515: F, t14758: F, t1524: F, t4435: F, t1197: F, t3696: F) -> (F, F, F, F, F, F) {
    let t14792 = t14776 + t14791;
    let t14793 = t14792 * t1537;
    let t14797 = F::cast_from(1.0_f64) / t4459 / t1527;
    let t14798 = t507 * t14797;
    let t14800 = F::cast_from(1.0_f64) / t4462 / t515;
    let t14801 = t14758 * t14800;
    let t14804 = t1524 * t4435;
    let t14807 = t14758 * t1537;
    let t14810 = t1197 * t3696;
    (t14793, t14798, t14801, t14804, t14807, t14810)
}
