//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1193/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1193<F: Float>(t2029: F, t7638: F, t2021: F, t1586: F, t2804: F, t32987: F, t33183: F, t34095: F, t34101: F, t34105: F, t34109: F, t34116: F, t34140: F, t9721: F, t9733: F, t9995: F) -> (F, F, F, F) {
    let t34518 = t2029 * t7638;
    let t34519 = t2021 * t34518;
    let t34520 = t1586 * t34519;
    let t34532 = -0.30952962962962962962e-2 * t34095 + 0.11607361111111111111e-2 * t34101 - 0.23214722222222222222e-2 * t34105 + 0.11607361111111111111e-2 * t34109 + 0.52083333333333333333e-2 * t2804 * t34520 + 0.52083333333333333333e-2 * t9721 * t9995 + 0.52083333333333333333e-2 * t9733 * t9995 + 0.20104166666666666667e-2 * t33183 * t9995 + 0.77382407407407407407e-3 * t34116 + 0.77382407407407407407e-3 * t34140 - 0.11607361111111111111e-2 * t32987;
    (t34518, t34519, t34520, t34532)
}
