//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 791/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk791<F: Float>(t41: F, t7705: F, t2794: F, t410: F, t2755: F, t1823: F, t963: F, t2747: F, t741: F, t1827: F, t1693: F, t898: F) -> (F, F, F, F, F, F, F) {
    let t7707 = F::new(2.0) * t41 * t7705;
    let t7708 = t410 * t2794;
    let t7720 = F::new(8.0) * t410 * t2755;
    let t7721 = t963 * t1823;
    let t7724 = F::cast_from(0.23392894490538584828e1_f64) * t2747 * t741;
    let t7725 = t963 * t1827;
    let t7727 = t898 * t1693;
    (t7707, t7708, t7720, t7721, t7724, t7725, t7727)
}
