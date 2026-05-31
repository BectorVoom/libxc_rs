//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1240/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1240<F: Float>(t3255: F, t5460: F, t5465: F, t544: F, t5481: F, t1319: F, t5457: F, t3809: F, t5458: F, t11633: F, t1961: F, t3762: F) -> (F, F, F, F, F, F, F) {
    let t16001 = F::cast_from(0.98556445e-3_f64) * t3255 * t5460;
    let t16003 = F::cast_from(0.19711289e-2_f64) * t3255 * t5465;
    let t16004 = t544 * t5481;
    let t16005 = t16004 * t1319;
    let t16006 = t5457 * t16005;
    let t16009 = t5458 * t3809;
    let t16010 = t5457 * t16009;
    let t16013 = t11633 * t1961;
    let t16014 = t16013 * t3762;
    (t16001, t16003, t16005, t16006, t16009, t16010, t16014)
}
