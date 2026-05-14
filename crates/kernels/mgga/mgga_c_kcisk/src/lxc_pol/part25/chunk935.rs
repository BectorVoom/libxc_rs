//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 935/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk935<F: Float>(t11612: F, t16105: F, t16107: F, t16108: F, t16111: F, t16114: F, t16118: F, t16122: F, t16124: F, t16125: F, t16128: F, t16132: F, t16135: F, t16295: F, t16298: F, t16300: F, t16303: F, t16304: F, t16562: F, t1689: F, t1809: F, t2399: F, t2505: F, t4794: F, t5089: F, t5134: F, t5172: F, t604: F, t674: F, t6941: F, t702: F) -> (F,) {
    let t16567 = -t16105 - t16107 + 0.46853067927761790996e-2 * t5089 * t16108 - 0.18741227171104716398e-1 * t11612 * t16111 + 0.46853067927761790996e-2 * t1809 * t16114 - 0.18741227171104716398e-1 * t5134 * t16118 - t16122 - t16124 + 0.93706135855523581992e-2 * t1809 * t16125 + 0.46853067927761790996e-2 * t1809 * t16128 + 0.28111840756657074598e-1 * t674 * t16132 + 0.14055920378328537299e-1 * t674 * t16135 - t604 * t16295 - t4794 * t2505 - 0.28111840756657074598e-1 * t16298 * t16300 - 0.18741227171104716398e-1 * t16303 * t16304 - t16562 * t702 - 2.0 * t1689 * t6941 - t2399 * t5172;
    (t16567,)
}
