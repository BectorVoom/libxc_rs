//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 762/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk762(t15363: f64, t69568: f64, t14236: f64, t14237: f64, t1528: f64, t2067: f64, t26: f64, t15388: f64, t68538: f64, t3154: f64, t38638: f64, t15266: f64, t16156: f64) -> (f64, f64, f64, f64, f64) {
    let t73807 = t69568 * t15363;
    let t73812 = t14236 * t14237 * t2067 * t26 * t1528;
    let t73814 = t68538 * t15388;
    let t73816 = t38638 * t3154;
    let t73817 = 0.19863479950205658386e-4_f64 * t73816;
    let t73819 = t16156 * t15266;
    (t73807, t73812, t73814, t73817, t73819)
}
