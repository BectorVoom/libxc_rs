//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1158/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1158<F: Float>(t1368: F, t16849: F, t1593: F, t5727: F, t12133: F, t1933: F, t12159: F, t1938: F, t4001: F, t613: F, t3971: F, t5691: F, t16830: F, t16833: F, t16838: F, t16842: F, t16845: F, t1930: F, t3991: F, t3995: F, t4003: F, t493: F) -> (F,) {
    let t16850 = t1368 * t16849;
    let t16852 = t1593 * t5727;
    let t16854 = t1368 * t16852 / 72.0;
    let t16857 = t12133 * t1933;
    let t16858 = t1368 * t16857;
    let t16861 = t12159 * t1938 * t4001;
    let t16862 = t613 * t16861;
    let t16866 = t5691 * t3971 / 162.0;
    let t16869 = t16830 * t16833 / 72.0 - t493 * t16838 / 144.0 + t16842 / 432.0 + t16845 - t1930 * t4003 / 18.0 + 7.0 / 432.0 * t16850 + t16854 + t5691 * t3991 / 54.0 - t16858 / 1296.0 - t1368 * t16862 / 16.0 - t16866 - t5691 * t3995 / 108.0;
    (t16869,)
}
