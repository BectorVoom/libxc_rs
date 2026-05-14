//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1063/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1063<F: Float>(t425: F, t4982: F, t4885: F, t4902: F, t99: F, t35: F, t409: F, t101: F, t4918: F, t1477: F, t1482: F, t1485: F, t390: F, t400: F) -> (F, F, F, F, F, F) {
    let t18789 = t4982 * t425;
    let t18791 = t4885 * t425;
    let t18794 = 1.0 / t99 / t4902;
    let t18806 = t35 * t409;
    let t18814 = 1.0 / t101 / t4918;
    let t18839 = 0.34367190188705947438e1 * t390 * t1482 * t1477 * t1485 * t400;
    (t18789, t18791, t18794, t18806, t18814, t18839)
}
