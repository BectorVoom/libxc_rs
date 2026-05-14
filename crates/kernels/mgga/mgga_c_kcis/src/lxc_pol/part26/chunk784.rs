//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 784/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk784<F: Float>(t16841: F, t493: F, t1930: F, t3974: F, t1369: F, t2469: F, t5714: F, t1368: F, t1593: F, t5727: F, t12133: F, t1933: F, t3971: F, t5691: F, t1377: F, t5713: F) -> (F, F, F, F, F, F, F, F) {
    let t16842 = t493 * t16841;
    let t16845 = t1930 * t3974 / 54.0;
    let t16848 = t2469 * t1369;
    let t16849 = t16848 * t5714;
    let t16850 = t1368 * t16849;
    let t16852 = t1593 * t5727;
    let t16854 = t1368 * t16852 / 72.0;
    let t16857 = t12133 * t1933;
    let t16858 = t1368 * t16857;
    let t16866 = t5691 * t3971 / 162.0;
    let t16884 = t5713 * t1377;
    (t16842, t16845, t16848, t16850, t16854, t16858, t16866, t16884)
}
