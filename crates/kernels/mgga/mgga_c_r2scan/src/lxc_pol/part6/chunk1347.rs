//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1347/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1347<F: Float>(t2155: F, t25184: F, t19890: F, t6093: F, t7605: F, t20524: F, t20528: F, t20532: F, t20539: F, t20542: F, t20552: F, t20561: F, t20563: F, t20567: F, t20573: F, t20576: F) -> (F,) {
    let t25456 = t2155 * t25184;
    let t25459 = t6093 * t19890 * t7605;
    let t25460 = 0.6112917064160653851e0 * t25459;
    let t25465 = 0.20958572791407956061e0 * t20524 - 0.16463622957338778996e-1 * t20528 + 0.87816964854445047168e-1 * t20532 - t20539 - 0.523649308946876022e0 * t20542 + 0.8781696485444504717e-1 * t25456 - t25460 + t20552 + t20561 - 0.16463622957338778996e-1 * t20563 + 0.82318114786693894983e-2 * t20567 - 0.48787202696913915093e-2 * t20573 + 0.19756347548806534796e0 * t20576;
    (t25465,)
}
