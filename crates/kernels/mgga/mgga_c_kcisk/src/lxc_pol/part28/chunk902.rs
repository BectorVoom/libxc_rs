//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 902/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk902<F: Float>(t2484: F, t4663: F, t1646: F, t6787: F, t15991: F, t11634: F, t6738: F, t1806: F, t6791: F, t1846: F, t2477: F, t2488: F, t5082: F, t1850: F, t6731: F, t6735: F, t696: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16037 = t4663 * t2484;
    let t16040 = t1646 * t6787;
    let t16061 = 0.18344444444444444444e-2 * t15991;
    let t16081 = t11634 * t6738;
    let t16084 = 0.93706135855523581992e-2 * t1806 * t6791;
    let t16088 = t1846 * t2477;
    let t16090 = t5082 * t2488;
    let t16105 = 0.93706135855523581992e-2 * t1850 * t6731;
    let t16107 = 0.93706135855523581992e-2 * t696 * t6735;
    (t16037, t16040, t16061, t16081, t16084, t16088, t16090, t16105, t16107)
}
