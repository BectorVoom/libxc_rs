//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1335/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1335(t4862: f64, t4864: f64, t5477: f64, t6085: f64, t6620: f64, t6751: f64, t6752: f64, t6754: f64, t7218: f64, t7219: f64, t7917: f64, t23594: f64, t4860: f64, t5473: f64, t5479: f64, t5481: f64, t6755: f64, t6756: f64, t7221: f64, t7915: f64, t8593: f64, t9: f64) -> f64 {
    let t23605 = -t6620 + 3.0_f64 * t7218 + t5477 - t6085 - 0.4303125e0_f64 * t7219 + t4864 + 0.1434375e0_f64 * t7917 + t4862 + 3.0_f64 * t6754 + 3.0_f64 * t6751 + 6.0_f64 * t6752;
    let tv4rho41 = -t5479 - 0.7171875e-1_f64 * t8593 + t4860 + 6.0_f64 * t6756 + t5481 - 0.7171875e-1_f64 * t7915 + t9 * t23594 + 3.0_f64 * t6755 + t5473 + 0.286875e0_f64 * t7221 + t23605;
    tv4rho41
}
