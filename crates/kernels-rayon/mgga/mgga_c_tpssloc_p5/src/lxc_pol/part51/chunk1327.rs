//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1327/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1327(t26502: f64, t3701: f64, t26114: f64, t8327: f64, t191: f64, t192: f64, t26138: f64, t19456: f64, t8326: f64, t26117: f64, t12725: f64, t1458: f64, t6514: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t120016 = t3701 * t26502;
    let t120067 = 2.0_f64 * t26114 * t8327;
    let t120071 = t26138 * t191 * t192;
    let t120120 = t19456 * t8326;
    let t120121 = 2.0_f64 * t120120;
    let t120122 = t26114 * t8326;
    let t120123 = 2.0_f64 * t120122;
    let t120124 = t26117 * t8326;
    let t120125 = 2.0_f64 * t120124;
    let t120130 = t12725 * t8326;
    let t120131 = 2.0_f64 * t120130;
    let t120145 = t6514 * t1458;
    (t120016, t120067, t120071, t120121, t120123, t120125, t120131, t120145)
}
