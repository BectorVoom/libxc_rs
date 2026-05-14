//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 986/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk986<F: Float>(t1724: F, t2943: F, t4667: F, t932: F, t13744: F, t1670: F, t9758: F, t2944: F, t4625: F, t934: F, t2952: F, t4657: F, t13712: F, t10218: F, t13710: F, t13714: F, t13723: F, t13732: F, t9700: F) -> (F, F, F, F, F, F, F) {
    let t13747 = t2943 * t1724;
    let t13750 = t932 * t4667;
    let t13767 = t932 * t13744;
    let t13771 = t9758 * t1670;
    let t13772 = t13771 * t2944;
    let t13774 = t2943 * t4625;
    let t13775 = t13774 * t934;
    let t13777 = t4657 * t2952;
    let t13781 = 0.18344444444444444444e-2 * t13712;
    let t13782 = -0.27516666666666666666e-2 * t9700 + 0.1982e-1 * t13767 + 0.1651e-1 * t13723 - 0.24765e-1 * t13732 + 0.14865e-1 * t13772 - t10218 - 0.1982e-1 * t13775 - 0.991e-2 * t13777 - 0.18344444444444444444e-2 * t13710 - 0.55033333333333333333e-2 * t13714 + t13781;
    (t13747, t13750, t13767, t13772, t13775, t13777, t13782)
}
