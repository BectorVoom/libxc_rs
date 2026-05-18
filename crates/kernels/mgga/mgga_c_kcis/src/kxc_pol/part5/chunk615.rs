//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 615/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk615<F: Float>(t3999: F, t494: F, t450: F, t740: F, t518: F, t1405: F, t532: F, t1401: F, t1420: F, t1444: F, t89: F) -> (F, F, F, F, F, F) {
    let t4000 = t494 * t3999;
    let t4016 = t740 * t450;
    let t4018 = F::new(0.46853067927761790996e-2) * t4016 * t518;
    let t4019 = t532 * t1405;
    let t4021 = t1401 * t1420;
    let t4023 = t89 * t1444;
    (t4000, t4016, t4018, t4019, t4021, t4023)
}
