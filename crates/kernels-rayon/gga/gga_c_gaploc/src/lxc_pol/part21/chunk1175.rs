//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1175/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1175(t2268: f64, t2440: f64, t2756: f64, t10135: f64, t6313: f64, t10132: f64, t6305: f64, t555: f64, t7861: f64, t888: f64, t7863: f64, t894: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31786 = 0.56910013271352299198e-1_f64 * t2268 * t2440 * t2756;
    let t31788 = 0.2276400530854091968e0_f64 * t6313 * t10135;
    let t31790 = 0.17073003981405689759e0_f64 * t6305 * t10132;
    let t31792 = 0.17073003981405689759e0_f64 * t6305 * t10135;
    let t31793 = t555 * t7861;
    let t31796 = 0.85365019907028448797e-1_f64 * t2268 * t31793 * t888;
    let t31799 = 0.28455006635676149599e-1_f64 * t2268 * t894 * t7863;
    (t31786, t31788, t31790, t31792, t31793, t31796, t31799)
}
