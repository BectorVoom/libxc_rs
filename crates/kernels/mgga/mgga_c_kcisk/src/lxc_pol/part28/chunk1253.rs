//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1253/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1253<F: Float>(t10000: F, t10005: F, t10014: F, t2804: F, t2807: F, t34081: F, t34140: F, t34537: F, t35181: F, t35184: F, t35187: F, t35203: F, t35206: F, t35416: F, t35463: F, t35469: F, t35476: F, t9725: F, t9991: F, t9995: F) -> (F,) {
    let t35491 = 0.10416666666666666667e-1 * t9991 * t10014 - 0.52083333333333333333e-2 * t35463 * t2807 - 0.23214722222222222222e-2 * t34081 + 0.52083333333333333333e-2 * t2804 * t35469 + 0.34822083333333333332e-2 * t35181 + 0.92858888888888888886e-2 * t35184 - 0.38691203703703703703e-3 * t35187 + 0.20104166666666666667e-2 * t9725 * t35476 - 0.10416666666666666667e-1 * t2804 * t35416 - 0.27777777777777777778e-1 * t10005 * t10014 + 0.10416666666666666667e-1 * t9991 * t9995 + 0.15476481481481481481e-2 * t34140 + 0.34722222222222222222e-2 * t34537 - 0.34822083333333333332e-2 * t35203 + 0.23214722222222222222e-2 * t35206 + 0.10416666666666666667e-1 * t10000 * t10014;
    (t35491,)
}
