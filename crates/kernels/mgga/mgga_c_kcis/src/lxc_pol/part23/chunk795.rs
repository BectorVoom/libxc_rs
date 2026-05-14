//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 795/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk795<F: Float>(t11491: F, t1897: F, t3781: F, t3820: F, t5481: F, t1319: F, t3809: F, t5513: F, t1330: F, t16078: F, t4714: F, t5567: F, t659: F, t5564: F, t16050: F, t11409: F, t11411: F, t11413: F, t11415: F, t11455: F, t11457: F, t11460: F, t16048: F, t16062: F, t16088: F) -> (F, F, F, F, F, F, F, F) {
    let t16131 = t11491 * t1897;
    let t16132 = t16131 * t3781;
    let t16134 = t3820 * t5481;
    let t16135 = t16134 * t1319;
    let t16137 = t5513 * t3809;
    let t16141 = t1330 * t16078;
    let t16142 = t4714 * t16141;
    let t16144 = t659 * t5567;
    let t16145 = 0.21908444444444444444e0 * t16144;
    let t16146 = t659 * t5564;
    let t16156 = 0.39862222222222222222e0 * t16050;
    let t16160 = -0.26574814814814814816e0 * t11409 + 0.66437037037037037038e-1 * t11411 - 0.19931111111111111111e0 * t11413 + 0.99655555555555555557e-1 * t11415 + 0.59793333333333333334e0 * t16088 + 0.11958666666666666667e1 * t16062 + 0.13287407407407407408e0 * t16048 - t16156 - 0.18257037037037037037e0 * t11455 + 0.54771111111111111111e-1 * t11457 + 0.18257037037037037037e-1 * t11460;
    (t16132, t16135, t16137, t16142, t16144, t16145, t16146, t16160)
}
