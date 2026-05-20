//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3005/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3005<F: Float>(t11710: F, t15974: F, t4899: F, t16183: F, t3153: F, t11866: F, t15794: F, t11671: F, t15925: F, t15752: F, t15917: F, t127: F, t15700: F, t15702: F, t4801: F) -> (F, F, F, F, F, F) {
    let t54907 = t4899 * t11710 * t15974;
    let t54909 = t16183 * t3153;
    let t54914 = t11866 * t15794;
    let t54916 = t15925 * t11671;
    let t54919 = t15917 * t15752;
    let t54925 = t15700 * t127 * t4801 * t15702;
    (t54907, t54909, t54914, t54916, t54919, t54925)
}
