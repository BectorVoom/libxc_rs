//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1243/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1243<F: Float>(t1020: F, t1083: F, t1085: F, t11106: F, t11108: F, t11111: F, t11113: F, t11981: F, t11983: F, t11985: F, t11987: F, t11989: F, t1310: F, t2410: F, t3388: F, t3390: F, t3394: F, t3652: F, t839: F, t8438: F) -> F {
    let t40986 = -F::cast_from(0.4355305902528e1_f64) * t11989 * t839 - F::cast_from(0.18428227254588e2_f64) * t11981 * t839 + F::cast_from(0.734774460522e2_f64) * t11983 * t839 - F::cast_from(0.7662840944824e2_f64) * t11985 * t839 + F::cast_from(0.3101306810232e2_f64) * t11987 * t839 - F::cast_from(0.9214113627294e1_f64) * t11106 * t1020 - F::cast_from(0.18428227254588e2_f64) * t11108 * t1020 - F::cast_from(0.18428227254588e2_f64) * t3388 * t2410 - F::cast_from(0.9214113627294e1_f64) * t11111 * t1020 - F::cast_from(0.18428227254588e2_f64) * t3390 * t2410 - F::cast_from(0.9214113627294e1_f64) * t1083 * t8438 - F::cast_from(0.9214113627294e1_f64) * t3652 * t1310 + F::cast_from(0.367387230261e2_f64) * t11113 * t1020 + F::cast_from(0.734774460522e2_f64) * t3394 * t2410 + F::cast_from(0.367387230261e2_f64) * t1085 * t8438;
    t40986
}
