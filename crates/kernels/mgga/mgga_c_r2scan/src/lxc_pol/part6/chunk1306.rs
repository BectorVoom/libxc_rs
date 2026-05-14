//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1306/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1306<F: Float>(t5135: F, t7460: F, t6149: F, t7457: F, t565: F, t9520: F, t8022: F, t8044: F, t6152: F, t7365: F, t20698: F, t1554: F, t19987: F, t20052: F, t20059: F, t20080: F, t20642: F, t2122: F, t2124: F, t24616: F, t2557: F, t2562: F, t2572: F, t2575: F, t2582: F, t360: F, t5066: F, t6127: F, t6141: F, t6198: F, t6450: F, t7994: F, t8012: F, t920: F) -> (F, F) {
    let t24665 = t5135 * t7460;
    let t24672 = t6149 * t7457;
    let t24674 = t565 * t9520;
    let t24682 = t8022 * t8044;
    let t24689 = t6152 * t7365;
    let t24695 = t20698 * t7460;
    let t24700 = 0.16463622957338778996e0 * t2557 * t2124 * t8012 * t6127 - 0.10401866088065122276e1 * t20052 - 0.69345773920434148506e0 * t20059 - 0.7801399566048841707e0 * t24665 * t360 * t2562 * t6198 + 0.13002332610081402845e0 * t19987 * t2575 - 0.69345773920434148506e0 * t24672 - 0.7801399566048841707e0 * t24674 * t6141 + 0.16463622957338778996e0 * t2122 * t2124 * t7994 * t1554 + 0.69345773920434148506e0 * t20080 - 0.13869154784086829701e1 * t24682 + 0.65854491829355115988e0 * t2557 * t2124 * t20642 * t920 * t24616 - 0.20803732176130244552e1 * t24689 - 0.43341108700271342816e-1 * t2582 * t360 * t2572 * t5066 + 0.31205598264195366828e1 * t24695 * t360 * t2562 * t6450;
    (t24695, t24700)
}
