//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 910/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk910<F: Float>(t16076: F, t1814: F, t11634: F, t6738: F, t1806: F, t6791: F, t2477: F, t3290: F, t1846: F, t2488: F, t5082: F, t11480: F, t2487: F, t4658: F, t2063: F, t5101: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16077 = t1814 * t16076;
    let t16081 = t11634 * t6738;
    let t16084 = 0.93706135855523581992e-2 * t1806 * t6791;
    let t16085 = t2477 * t3290;
    let t16088 = t1846 * t2477;
    let t16090 = t5082 * t2488;
    let t16092 = t2488 * t3290;
    let t16095 = t11480 * t2487;
    let t16096 = t16095 * t4658;
    let t16099 = t5101 * t2063;
    (t16077, t16081, t16084, t16085, t16088, t16090, t16092, t16096, t16099)
}
