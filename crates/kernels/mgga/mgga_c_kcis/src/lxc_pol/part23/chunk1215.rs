//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1215/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1215<F: Float>(t15956: F, t97793: F, t17383: F, t7952: F, t27517: F, t5910: F, t2060: F, t1467: F, t4294: F, t1928: F, t4254: F, t27521: F, sigma2: F) -> (F, F, F, F, F) {
    let t97794 = t97793 * t15956;
    let t97796 = t7952 * t17383;
    let t97798 = t27517 * t5910;
    let t97800 = sigma2 * t2060;
    let t97801 = t1467 * t97800;
    let t97802 = t97801 * t4294;
    let t97804 = t4254 * t1928;
    let t97805 = t97804 * t27521;
    (t97794, t97796, t97798, t97802, t97805)
}
