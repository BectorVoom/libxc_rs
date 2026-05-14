//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1158/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1158<F: Float>(t1818: F, t21115: F, t720: F, t61: F, t5376: F, t5942: F, t5375: F, t4715: F, t735: F, t745: F, t1668: F, t1727: F, t5226: F, t615: F, t1757: F, t5464: F, t591: F) -> (F, F, F, F, F, F, F) {
    let t21117 = t1818 * t21115 * t720;
    let t21119 = 0.69350015718254262348e2 * t61 * t21117;
    let t21120 = t5376 * t5942;
    let t21121 = t5375 * t21120;
    let t21125 = 0.67471172535210825684e-1 * t735 * t4715 * t745;
    let t21129 = 0.40647071359999999999e-1 * t5226 * t615 * t1668 * t1727;
    let t21133 = 0.21076259223703703703e-1 * t1757 * t615 * t5464 * t591;
    (t21117, t21119, t21120, t21121, t21125, t21129, t21133)
}
