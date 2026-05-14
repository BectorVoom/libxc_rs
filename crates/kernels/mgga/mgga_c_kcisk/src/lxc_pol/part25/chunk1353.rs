//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1353/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1353<F: Float>(t1310: F, t5508: F, t33198: F, t36707: F, t2023: F, t33197: F, t7261: F, t7644: F, t2028: F, t34403: F, t47649: F, t786: F, t2647: F, t5437: F, t2803: F, t60823: F, t79: F) -> (F, F, F, F, F) {
    let t117652 = t1310 * t5508;
    let t117654 = t117652 * t36707 * t33198;
    let t117663 = t7261 * t33197 * t7644 * t2023;
    let t117668 = t7261 * t34403 * t7644 * t2028;
    let t117671 = t786 * t47649;
    let t117674 = t7261 * t117671 * t2647 * t5437;
    let t117683 = t60823 * t79 * t2803;
    (t117654, t117663, t117668, t117674, t117683)
}
