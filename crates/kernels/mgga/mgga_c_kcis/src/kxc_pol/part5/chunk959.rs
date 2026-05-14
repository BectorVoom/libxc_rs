//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 959/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk959<F: Float>(t1369: F, t2469: F, t5714: F, t1368: F, t1593: F, t5727: F, t12133: F, t1933: F, t3971: F, t5691: F, t1377: F, t5713: F, t498: F, t12217: F, t3977: F, t736: F) -> (F, F, F, F, F, F, F, F) {
    let t16848 = t2469 * t1369;
    let t16849 = t16848 * t5714;
    let t16850 = t1368 * t16849;
    let t16852 = t1593 * t5727;
    let t16854 = t1368 * t16852 / 72.0;
    let t16857 = t12133 * t1933;
    let t16858 = t1368 * t16857;
    let t16866 = t5691 * t3971 / 162.0;
    let t16884 = t5713 * t1377;
    let t16892 = t5713 * t498;
    let t16901 = t12217 * t498;
    let t16905 = t736 * t3977;
    (t16850, t16854, t16858, t16866, t16884, t16892, t16901, t16905)
}
