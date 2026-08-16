//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1053/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1053(t3406: f64, t8133: f64, t2579: f64, t3412: f64, t1615: f64, t2962: f64, t11295: f64, t12007: f64, t11282: f64, t11285: f64, t11610: f64, t11289: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30324 = t3406 * t8133;
    let t30325 = t2579 * t3412 * t30324;
    let t30472 = t2962 * t1615;
    let t33091 = 8.0_f64 * t11295;
    let t33093 = 2.0_f64 * t12007;
    let t33094 = 2.0_f64 * t11282;
    let t33095 = 8.0_f64 * t11285;
    let t33096 = 2.0_f64 * t11610;
    let t33097 = 4.0_f64 * t11289;
    (t30324, t30325, t30472, t33091, t33093, t33094, t33095, t33096, t33097)
}
