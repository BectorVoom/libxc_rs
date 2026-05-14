//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 993/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk993<F: Float>(t1749: F, t1774: F, t303: F, t6614: F, t7726: F, t26679: F, t6272: F, t4947: F, t1709: F, t27778: F, t26686: F) -> (F, F, F, F, F, F, F, F) {
    let t28966 = t1749 * t1774;
    let t28967 = t303 * t28966;
    let t28973 = t7726 * t6614;
    let t28974 = t303 * t28973;
    let t28983 = t26679 * t6272;
    let t28984 = t4947 * t28983;
    let t28987 = t27778 * t1709;
    let t28988 = t26686 * t28987;
    (t28966, t28967, t28973, t28974, t28983, t28984, t28987, t28988)
}
