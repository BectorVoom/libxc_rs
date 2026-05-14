//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1374/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1374<F: Float>(t34465: F, t9721: F, t118141: F, t5014: F, t5439: F, t786: F, t33227: F, t6763: F, t10000: F, t112523: F, t112810: F, t113181: F, t116825: F, t116836: F, t118138: F, t18328: F, t33183: F, t33188: F, t33193: F, t34452: F, t34469: F, t34474: F, t34477: F, t9748: F, t9991: F, t9995: F) -> (F, F, F) {
    let t118174 = 0.34722222222222222222e-2 * t9721 * t34465;
    let t118180 = t5014 * t118141;
    let t118184 = t786 * t5439;
    let t118185 = t5014 * t118184;
    let t118187 = t118185 * t6763 * t33227;
    let t118200 = 0.20104166666666666667e-2 * t112810 * t9995 + 0.52083333333333333333e-2 * t10000 * t33188 + t118174 - 0.46429444444444444444e-2 * t116825 - 0.51588271604938271604e-3 * t112523 + 0.10317654320987654321e-2 * t116836 - 0.34722222222222222222e-2 * t113181 * t118138 - 0.69444444444444444444e-2 * t113181 * t118180 * t18328 - 0.69444444444444444444e-2 * t113181 * t118187 + 0.10416666666666666667e-1 * t34474 * t9748 + 0.40208333333333333334e-2 * t33183 * t34469 + 0.10416666666666666667e-1 * t34477 * t9748 + 0.52083333333333333333e-2 * t9991 * t33193 + 0.10416666666666666667e-1 * t34452 * t9748;
    (t118184, t118187, t118200)
}
