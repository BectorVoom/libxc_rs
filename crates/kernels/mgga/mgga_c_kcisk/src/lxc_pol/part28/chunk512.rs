//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 512/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk512<F: Float>(t4636: F, t4722: F, t1675: F) -> (F, F, F, F) {
    let t4769 = 0.40256666666666666667e0 * t4636;
    let t4776 = 0.137975e0 * t4722;
    let t4786 = t1675 * t1675;
    let t4787 = 1.0 / t4786;
    (t4769, t4776, t4786, t4787)
}
