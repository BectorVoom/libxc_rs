//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1417/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1417(t12214: f64, t205: f64, t116: f64, t547: f64, t1307: f64, t212: f64, t2586: f64, t535: f64, t9534: f64, t9538: f64, t1337: f64, t562: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12215 = t205 * t12214;
    let t12225 = t547 * t116;
    let t12226 = t212 * t1307;
    let t12227 = t12225 * t12226;
    let t12228 = t2586 * t12227;
    let t12236 = 0.13888888888888888889e-3_f64 * t9534 * t535 * t9538;
    let t12247 = t1337 * t1337;
    let t12248 = 1.0_f64 / t12247;
    let t12249 = t12248 * t562;
    (t12215, t12225, t12227, t12228, t12236, t12247, t12248, t12249)
}
