//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 881/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk881<F: Float>(t10728: F, t11696: F, t10710: F, t7261: F, t10708: F, t2124: F, t8070: F, t3295: F, t3308: F, t7629: F, t2184: F, t8156: F, t1592: F, t8160: F, t7615: F, t2196: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11697 = t10728 * t11696;
    let t11699 = t10710 * t7261;
    let t11700 = t10708 * t11699;
    let t11702 = t2124 * t8070;
    let t11703 = t3295 * t11702;
    let t11705 = t3308 * t7629;
    let t11706 = t2184 * t11705;
    let t11708 = t3308 * t8156;
    let t11709 = t1592 * t11708;
    let t11711 = t3308 * t8160;
    let t11712 = t1592 * t11711;
    let t11714 = t3308 * t7615;
    let t11715 = t2196 * t11714;
    (t11697, t11699, t11700, t11702, t11703, t11705, t11706, t11708, t11709, t11711, t11712, t11714, t11715)
}
