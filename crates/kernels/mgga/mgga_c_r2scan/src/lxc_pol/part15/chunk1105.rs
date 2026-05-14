//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1105/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1105<F: Float>(t1020: F, t1083: F, t1085: F, t11106: F, t11108: F, t11111: F, t11113: F, t11981: F, t11983: F, t11985: F, t11987: F, t11989: F, t1310: F, t2410: F, t3388: F, t3390: F, t3394: F, t3652: F, t839: F, t8438: F) -> (F,) {
    let t40986 = -0.4355305902528e1 * t11989 * t839 - 0.18428227254588e2 * t11981 * t839 + 0.734774460522e2 * t11983 * t839 - 0.7662840944824e2 * t11985 * t839 + 0.3101306810232e2 * t11987 * t839 - 0.9214113627294e1 * t11106 * t1020 - 0.18428227254588e2 * t11108 * t1020 - 0.18428227254588e2 * t3388 * t2410 - 0.9214113627294e1 * t11111 * t1020 - 0.18428227254588e2 * t3390 * t2410 - 0.9214113627294e1 * t1083 * t8438 - 0.9214113627294e1 * t3652 * t1310 + 0.367387230261e2 * t11113 * t1020 + 0.734774460522e2 * t3394 * t2410 + 0.367387230261e2 * t1085 * t8438;
    (t40986,)
}
