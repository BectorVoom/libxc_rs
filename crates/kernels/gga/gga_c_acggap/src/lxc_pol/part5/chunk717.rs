//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 717/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk717<F: Float>(t1748: F, t360: F, t1181: F, t1532: F, t372: F, t1165: F, t1552: F, t407: F, t495: F, t1753: F, t322: F, t1163: F, t3194: F, t301: F, t513: F, t944: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5710 = t1748 * t360;
    let t5712 = t1181 * t1532 * t5710;
    let t5715 = t1748 * t372;
    let t5717 = t1165 * t1552 * t5715;
    let t5720 = t407 * t495;
    let t5722 = t1165 * t1532 * t5720;
    let t5725 = t1753 * t322;
    let t5726 = t1532 * t5725;
    let t5727 = t1181 * t5726;
    let t5728 = t1163 * t5727;
    let t5730 = t1748 * t322;
    let t5732 = t1165 * t1532 * t5730;
    let t5733 = t3194 * t5732;
    let t5735 = t1748 * t301;
    let t5737 = t1165 * t1532 * t5735;
    let t5740 = t944 * t513;
    (t5710, t5712, t5715, t5717, t5720, t5722, t5725, t5726, t5727, t5728, t5730, t5732, t5733, t5735, t5737, t5740)
}
