//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2234/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2234<F: Float>(t17611: F, t3109: F, t135: F, t17737: F, t973: F, t10949: F, t17667: F, t17607: F, t3053: F, t3047: F, t5904: F, t18030: F, t3103: F) -> (F, F, F, F, F, F) {
    let t61695 = t3109 * t17611;
    let t61699 = t973 * t135 * t17737;
    let t61705 = t10949 * t17667;
    let t61708 = t17607 * t3053;
    let t61710 = t5904 * t3047;
    let t61713 = t18030 * t3103;
    (t61695, t61699, t61705, t61708, t61710, t61713)
}
