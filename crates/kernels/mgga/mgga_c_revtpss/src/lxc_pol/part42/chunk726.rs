//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 726/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk726<F: Float>(t1045: F, t373: F, t4866: F, t1042: F, t1065: F, t905: F, t1469: F, t999: F, t1032: F, t1647: F, t1040: F) -> (F, F, F, F, F, F, F, F) {
    let t4868 = t373 * t4866 * t1045;
    let t4869 = t1042 * t4868;
    let t4872 = t1065 * t905;
    let t4873 = t1469 * t999;
    let t4874 = t4872 * t4873;
    let t4875 = t1042 * t4874;
    let t4878 = t1647 * t1032;
    let t4879 = t4878 * t1040;
    (t4868, t4869, t4872, t4873, t4874, t4875, t4878, t4879)
}
