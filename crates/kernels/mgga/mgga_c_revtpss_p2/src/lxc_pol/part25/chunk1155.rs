//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1155/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1155<F: Float>(t1983: F, t25604: F, t1043: F, t7161: F, t1089: F, t378: F, t7150: F, t8521: F, t7152: F, t7135: F, t999: F, t7145: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25605 = t1983 * t25604;
    let t25606 = t7161 * t1043;
    let t25607 = t25606 * t1089;
    let t25610 = t7150 * t378;
    let t25611 = t25610 * t8521;
    let t25612 = t7152 * t1043;
    let t25613 = t25612 * t1089;
    let t25616 = t7135 * t999;
    let t25617 = t7145 * t25616;
    (t25605, t25606, t25607, t25610, t25611, t25612, t25613, t25616, t25617)
}
