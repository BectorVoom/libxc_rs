//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 891/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk891<F: Float>(t17537: F, t424: F, t1765: F, t496: F, t429: F, t8: F, t11928: F, t1220: F, t15012: F, t15099: F, t15101: F, t15105: F, t15108: F, t15138: F, t1554: F, t1570: F, t1579: F, t17465: F, t17471: F, t17504: F, t17516: F, t17527: F, t17531: F, t17536: F, t3980: F, t4230: F, t4297: F, t4536: F, t5098: F, t5103: F, t5441: F) -> (F, F, F, F) {
    let t17539 = 1.0 / t424 / t17537;
    let t17542 = t1765 * t496;
    let t17543 = t17539 * t8 * t429 * t17542;
    let t17548 = -4.0 / 3.0 * t1220 * t17465 + t17471 - t17504 + t15012 * t1579 / 2.0 + t4536 * t5098 / 2.0 + 2.0 / 3.0 * t4536 * t5103 - 4.0 / 3.0 * t4230 * t5098 - 16.0 / 9.0 * t4230 * t5103 - 100.0 / 27.0 * t4297 * t17516 - t11928 / 9.0 - 0.77534644304710291488e-2 * t3980 * t15138 * t1554 + 100.0 / 27.0 * t15099 - 50.0 / 3.0 * t15101 + 20000.0 / 81.0 * t15105 - t17527 - t17531 + 100.0 / 81.0 * t15108 + 4000000.0 / 243.0 * t17536 * t17543 + 44.0 / 3.0 * t1570 * t5441;
    (t17539, t17542, t17543, t17548)
}
