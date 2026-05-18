//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1247/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1247<F: Float>(t1301: F, t6888: F, t1640: F, t6896: F, t446: F, t6298: F, t911: F, t6884: F, t1300: F, t7570: F, t1295: F, t6294: F) -> (F, F, F, F, F, F) {
    let t20856 = t6888 * t1301;
    let t20858 = t6896 * t1640;
    let t20859 = t446 * t20858;
    let t20861 = t911 * t6298;
    let t20863 = t911 * t6884;
    let t20865 = t1300 * t7570;
    let t20866 = t446 * t20865;
    let t20869 = t6294 * t1295;
    (t20856, t20859, t20861, t20863, t20866, t20869)
}
