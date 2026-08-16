//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1658/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1658(t12214: f64, t205: f64, t12156: f64, t210: f64, t214: f64, t1307: f64, t213: f64, t221: f64, t3719: f64, t116: f64, t547: f64, t212: f64) -> (f64, f64, f64, f64, f64) {
    let t12215 = t205 * t12214;
    let t12217 = t210 * t214 * t12156;
    let t12220 = t213 * t1307;
    let t12222 = t221 * t12220 * t3719;
    let t12225 = t547 * t116;
    let t12226 = t212 * t1307;
    (t12215, t12217, t12222, t12225, t12226)
}
