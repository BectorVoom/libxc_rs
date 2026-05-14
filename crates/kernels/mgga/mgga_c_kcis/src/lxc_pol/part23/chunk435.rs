//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 435/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk435<F: Float>(t1307: F, t1419: F, t3766: F, t1444: F, t544: F, t1471: F, t2642: F, t1472: F, t2645: F, t1317: F, t1319: F) -> (F, F, F, F, F, F) {
    let t3767 = t1307 * t1419;
    let t3768 = t3766 * t3767;
    let t3771 = t544 * t1444;
    let t3773 = t1471 * t3771 * t2642;
    let t3777 = t1471 * t1472 * t2645;
    let t3780 = t1317 * t544;
    let t3781 = t1319 * t1319;
    (t3767, t3768, t3773, t3777, t3780, t3781)
}
