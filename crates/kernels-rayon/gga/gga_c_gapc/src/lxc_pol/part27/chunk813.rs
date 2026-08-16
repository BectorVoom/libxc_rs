//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 813/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk813(t2562: f64, t327: f64, t8820: f64, t2560: f64, t2568: f64, t291: f64, t7549: f64, t7547: f64, t871: f64, t903: f64, t2526: f64, t9430: f64, t9433: f64, t9436: f64, t9440: f64, t9442: f64, t9445: f64, t9447: f64, t9449: f64, t9451: f64) -> f64 {
    let t9454 = t8820 * t327 * t2562;
    let t9455 = t2560 * t9454;
    let t9457 = t2568 * t9454;
    let t9460 = t8820 * t291 * t7549;
    let t9461 = t7547 * t9460;
    let t9463 = t871 * t903;
    let t9464 = t9463 * t2526;
    let t9466 = -0.27801896084645508334e-2_f64 * t9430 + 0.12163329537032409896e-2_f64 * t9433 - 0.42270452978984302532e-6_f64 * t9436 - 0.14480154210752868924e-5_f64 * t9440 + 0.17376185052903442709e-3_f64 * t9442 + 0.687148483626368822e-6_f64 * t9445 - 0.2318836277704281739e-4_f64 * t9447 + 0.16908181191593721013e-4_f64 * t9449 - 0.33816362383187442026e-4_f64 * t9451 + 0.1374296967252737644e-6_f64 * t9455 - 0.18326250058315256483e-6_f64 * t9457 - 0.45775879823985672486e-6_f64 * t9461 - 0.12357942809624928455e-3_f64 * t9464;
    t9466
}
