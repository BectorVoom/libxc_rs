//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2234/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2234<F: Float>(t1045: F, t23820: F, t373: F, t1042: F, t11632: F, t23641: F, t11250: F, t1668: F, t6244: F) -> (F, F, F, F, F, F, F) {
    let t23822 = t373 * t23820 * t1045;
    let t23823 = t1042 * t23822;
    let t23829 = t23641 * t11632;
    let t23830 = t1042 * t23829;
    let t23833 = t23641 * t11250;
    let t23834 = t1042 * t23833;
    let t23837 = t6244 * t1668;
    (t23822, t23823, t23829, t23830, t23833, t23834, t23837)
}
