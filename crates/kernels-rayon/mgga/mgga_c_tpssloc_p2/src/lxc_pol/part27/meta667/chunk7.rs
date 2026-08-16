//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2350/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2350(t91486: f64, t225: f64, t26329: f64, t26229: f64, t81375: f64, t1324: f64, t254: f64, t12020: f64, t1386: f64, t16439: f64, t1843: f64, t22656: f64, t22670: f64, t26224: f64, t26226: f64, t5210: f64, t5325: f64, t5326: f64, t568: f64, t6955: f64, t6992: f64, t6993: f64, t80704: f64) -> f64 {
    let t91487 = 0.16449340668482264365e-1_f64 * t91486;
    let t91488 = t26329 * t225;
    let t91491 = t26229 * t225;
    let t91496 = 0.25587863262083522346e0_f64 * t81375;
    let t91505 = t1324 * t254;
    let t91512 = -12.0_f64 * t12020 * t26224 * t5325 * t6992 + 2.0_f64 * t5210 * t568 * t6955 - 2.0_f64 * t1386 * t91488 - 2.0_f64 * t1386 * t91491 - 2.0_f64 * t16439 * t6993 - t1843 * t80704 + 4.0_f64 * t22656 * t5326 + 4.0_f64 * t22670 * t5326 - 12.0_f64 * t26226 * t91505 + t91487 - t91496;
    t91512
}
