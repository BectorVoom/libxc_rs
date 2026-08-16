//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2357/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2357(t26502: f64, t532: f64, t1983: f64, t6879: f64, t2314: f64, t26142: f64, t4034: f64, t1266: f64, t26135: f64, t652: f64, t24987: f64, t6997: f64) -> (f64, f64, f64, f64, f64) {
    let t91620 = t532 * t26502;
    let t91623 = 6.0_f64 * t1983 * t91620 * t6879;
    let t91625 = 4.0_f64 * t2314 * t26142;
    let t91627 = 4.0_f64 * t4034 * t26142;
    let t91630 = 4.0_f64 * t652 * t1266 * t26135;
    let t91637 = 2.0_f64 * t24987 * t6997;
    (t91623, t91625, t91627, t91630, t91637)
}
