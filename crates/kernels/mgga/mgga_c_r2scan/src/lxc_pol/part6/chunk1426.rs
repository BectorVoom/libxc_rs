//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1426/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1426<F: Float>(t22591: F, t2823: F, t18934: F, t18941: F, t18973: F, t18975: F, t23719: F, t23785: F, t23788: F, t23824: F, t26899: F, t26901: F, t765: F, t2482: F, t6027: F, t6029: F) -> (F, F) {
    let t26903 = t2823 * t22591;
    let t26911 = -t18934 - 0.2025780996e0 * t26899 - 0.4051561992e0 * t26901 - 0.2025780996e0 * t26903 + t18941 + 0.675260332e-1 * t765 * t23785 + 0.2025780996e0 * t765 * t23788 + 0.2025780996e0 * t765 * t23824 + t23719 - t18973 - t18975;
    let t26917 = t6027 * t2482 * t6029;
    (t26911, t26917)
}
