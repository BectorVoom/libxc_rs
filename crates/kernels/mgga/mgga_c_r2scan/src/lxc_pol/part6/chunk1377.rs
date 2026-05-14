//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1377/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1377<F: Float>(t1568: F, t24161: F, t6155: F, t2195: F, t7983: F, t2583: F, t3433: F, t19851: F, t571: F, t6118: F, t7996: F, t2578: F, t19852: F, t7494: F, t7974: F, t6303: F, t7984: F) -> (F, F, F, F, F, F, F) {
    let t26135 = t6155 * t1568 * t24161;
    let t26141 = t2195 * t7983;
    let t26145 = t3433 * t2583;
    let t26146 = t571 * t19851 * t26145;
    let t26147 = 0.19043987679069580388e-1 * t26146;
    let t26148 = t6118 * t7996;
    let t26150 = t3433 * t2578;
    let t26151 = t19852 * t26150;
    let t26153 = t7494 * t7974;
    let t26155 = t7984 * t6303;
    (t26135, t26141, t26147, t26148, t26151, t26153, t26155)
}
