//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 589/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk589<F: Float>(t2059: F, t443: F, t3859: F, t1049: F, t1390: F, t212: F, t23: F, t6: F, t161: F, t1383: F, t5601: F, t1048: F, t9: F) -> (F, F, F, F, F, F, F, F) {
    let t5802 = t443 * t2059;
    let t5804 = t3859 * t2059;
    let t5808 = t1049 * t1390;
    let t5814 = 1.0 / t23 / t212;
    let t5815 = t6 * t5814;
    let t5816 = t161 * t5815;
    let t5817 = t1383 * t5601;
    let t5821 = 1.0 / t9 / t1048;
    (t5802, t5804, t5808, t5814, t5815, t5816, t5817, t5821)
}
