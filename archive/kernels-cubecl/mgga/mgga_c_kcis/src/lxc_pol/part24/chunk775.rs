//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 775/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk775<F: Float>(t11061: F, t1254: F, t1251: F, t25: F, t2887: F, t3530: F, t993: F, t1259: F, t2880: F) -> (F, F, F, F) {
    let t11062 = t11061 * t1254;
    let t11063 = t1251 * t11062;
    let t11068 = t25 * t2887;
    let t11072 = t993 * t3530;
    let t11081 = t2880 * t1259;
    (t11063, t11068, t11072, t11081)
}
