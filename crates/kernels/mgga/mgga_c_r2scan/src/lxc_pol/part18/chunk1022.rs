//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1022/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1022<F: Float>(t1020: F, t1087: F, t1089: F, t1091: F, t11987: F, t11989: F, t12660: F, t12662: F, t12664: F, t12666: F, t12668: F, t2410: F, t2956: F, t2958: F, t3388: F, t3402: F, t3406: F, t3664: F, t3668: F, t839: F, t9707: F) -> (F,) {
    let t42677 = -0.3831420472412e2 * t1087 * t9707 + 0.3101306810232e2 * t11987 * t1020 + 0.3101306810232e2 * t3664 * t2410 + 0.1550653405116e2 * t3402 * t2956 + 0.1550653405116e2 * t1089 * t9707 - 0.4355305902528e1 * t11989 * t1020 - 0.4355305902528e1 * t3668 * t2410 - 0.2177652951264e1 * t3406 * t2956 - 0.2177652951264e1 * t1091 * t9707 - 0.9214113627294e1 * t12660 * t839 + 0.367387230261e2 * t12662 * t839 - 0.3831420472412e2 * t12664 * t839 + 0.1550653405116e2 * t12666 * t839 - 0.2177652951264e1 * t12668 * t839 + 0.734774460522e2 * t3388 * t2958;
    (t42677,)
}
