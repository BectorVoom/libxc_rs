//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1119/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1119(t22988: f64, t3174: f64, t3176: f64, t487: f64, t1228: f64, t300: f64, t19107: f64, t22971: f64, t19116: f64, t54: f64, t8253: f64, t1167: f64, t179: f64, t19150: f64, t404: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22989 = 0.28582678745379824648e-3_f64 * t22988;
    let t23007 = t3174 * t487 * t3176;
    let t23008 = t23007 / 72.0_f64;
    let t23054 = t300 * t1228;
    let t23075 = t19107 * t22971;
    let t23081 = t19116 * t22971;
    let t23213 = t54 * t8253;
    let t23272 = t404 * t179 * t19150 * t1167;
    (t22989, t23008, t23054, t23075, t23081, t23213, t23272)
}
