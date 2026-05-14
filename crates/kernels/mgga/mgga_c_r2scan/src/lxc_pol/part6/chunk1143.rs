//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1143/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1143<F: Float>(t20837: F, t5149: F, t5143: F, t6465: F, t5169: F, t6407: F, t2195: F, t6474: F, t1543: F, t6212: F, t6211: F, t20137: F, t6480: F, t6481: F, t2097: F, t2167: F) -> (F, F, F, F, F, F, F) {
    let t20838 = t20837 * t5149;
    let t20840 = t6465 * t5143;
    let t20845 = t6407 * t5169;
    let t20852 = t2195 * t6474;
    let t20853 = t6212 * t1543;
    let t20855 = t20852 * t6211 * t20853;
    let t20858 = t6480 * t20137 * t6481;
    let t20860 = t2167 * t2097;
    (t20838, t20840, t20845, t20852, t20855, t20858, t20860)
}
