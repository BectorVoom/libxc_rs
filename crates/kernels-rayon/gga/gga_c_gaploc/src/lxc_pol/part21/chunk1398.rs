//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1398/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1398(t1: f64, t106: f64, t12000: f64, t192: f64, t12075: f64, t12098: f64, t12124: f64, t1580: f64, t1599: f64, t1628: f64, t31018: f64, t34817: f64, t34821: f64, t34823: f64, t34827: f64, t34829: f64, t34831: f64, t34839: f64, t34842: f64, t3709: f64, t38281: f64, t38313: f64, t4598: f64, t4818: f64, t4820: f64, t536: f64, t544: f64, t568: f64, t574: f64, t597: f64, t600: f64) -> f64 {
    let t38759 = t12000 * t1 * t106 * t192;
    let t38762 = -0.47667319935800568892e0_f64 * t1599 * t12098 + 0.61348681526273199482e1_f64 * t1580 * t12075 + 0.61348681526273199482e1_f64 * t597 * t1628 * t12124 - 0.1022478025437886658e1_f64 * t574 * t4598 * t3709 + 0.23005755572352449806e1_f64 * t597 * t568 * t600 * t38313 + t34817 - t34821 + 0.23833659967900284446e0_f64 * t544 * t4818 * t4820 * t38281 + 0.71500979903700853338e0_f64 * t536 * t38759 + t34823 - t34827 - t34829 - t34831 - t34839 + t34842 + t31018;
    t38762
}
