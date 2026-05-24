//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 699/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk699<F: Float>(t1736: F, t4953: F, t1744: F, t4929: F, t4956: F, t633: F, t630: F, t4957: F, t45: F, t4920: F, t1704: F, t4907: F) -> (F, F, F, F, F) {
    let t10902 = F::new(1.0) / t4953 / t1736;
    let t10903 = t4929 * t1744;
    let t10906 = F::new(1.0) / t4956 / t633;
    let t10907 = t10902 * t10903 * t10906;
    let t10913 = F::new(1.0) / t4953 / t630;
    let t10915 = t10913 * t10903 * t4957;
    let t10918 = t45 * t4920;
    let t10924 = F::new(1.0) / t4907 / t1704;
    (t10903, t10907, t10915, t10918, t10924)
}
