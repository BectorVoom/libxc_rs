//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1448/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1448<F: Float>(t18806: F, t40: F, t298: F, t308: F, t1000: F, t10410: F, t10417: F, t10423: F, t1216: F, t1217: F, t23462: F, t23473: F, t2900: F, t2904: F, t2916: F, t2920: F, t295: F, t305: F, t31608: F, t35: F, t803: F, t806: F, t810: F, t811: F, t8319: F, t8340: F, t9627: F, t9635: F, t990: F, t997: F) -> (F, F, F) {
    let t34958 = 6.0 * t40 + 12.0 * t18806;
    let t34959 = t298 * t34958;
    let t34967 = -t308 * t34958;
    let t34994 = 50.0 / 81.0 * t803 * t10410 - 25.0 / 9.0 * t803 * t10417 + 5.0 / 3.0 * t295 * t34959 - 2200.0 / 81.0 * t10423 * t811 - 25.0 / 3.0 * t997 * t9635 + 5.0 / 3.0 * t305 * t34967 - 10.0 / 9.0 * t23462 * t2900 * t35 * t1216 - 10.0 / 9.0 * t23462 * t990 * t2904 * t806 + 10.0 / 3.0 * t8319 * t1217 * t2904 + 100.0 / 9.0 * t31608 * t9627 + 10.0 / 9.0 * t23473 * t2916 * t35 * t1216 - 10.0 / 9.0 * t23473 * t1000 * t2920 * t810 - 10.0 / 3.0 * t8340 * t1217 * t2920;
    (t34959, t34967, t34994)
}
