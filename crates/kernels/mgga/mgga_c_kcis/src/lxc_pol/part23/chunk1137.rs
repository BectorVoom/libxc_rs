//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1137/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1137<F: Float>(t4479: F, t6220: F, t1505: F, t17306: F, t1628: F, t18266: F, t1610: F, t6183: F, t2104: F, t4463: F, t110: F, t494: F) -> (F, F, F, F, F, F) {
    let t52930 = t6220 * t4479;
    let t52933 = t17306 * t1505;
    let t52955 = t18266 * t1628;
    let t53436 = t6183 * t1610;
    let t53551 = t2104 * t4463;
    let t54162 = t110 * t494;
    (t52930, t52933, t52955, t53436, t53551, t54162)
}
