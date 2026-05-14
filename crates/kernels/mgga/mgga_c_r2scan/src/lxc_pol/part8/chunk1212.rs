//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1212/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1212<F: Float>(t2530: F, t538: F, t6191: F, t6194: F, t19890: F, t6093: F, t7605: F, t2654: F, t6212: F, t20237: F, t6211: F, t2625: F, t20852: F, t2634: F, t20589: F, t2612: F) -> (F, F, F, F, F, F) {
    let t25405 = t6191 * t538 * t2530 * t6194;
    let t25406 = 0.43371823197556470519e-3 * t25405;
    let t25459 = t6093 * t19890 * t7605;
    let t25460 = 0.6112917064160653851e0 * t25459;
    let t25480 = t6212 * t2654;
    let t25482 = t20237 * t6211 * t25480;
    let t25483 = 0.57131963037208741166e-1 * t25482;
    let t25486 = t6212 * t2625;
    let t25488 = t20852 * t6211 * t25486;
    let t25499 = t6212 * t2634;
    let t25501 = t20589 * t6211 * t25499;
    let t25503 = t6212 * t2612;
    (t25406, t25460, t25483, t25488, t25501, t25503)
}
