//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 810/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk810<F: Float>(t2562: F, t327: F, t8820: F, t2560: F, t2568: F, t291: F, t7549: F, t7547: F, t871: F, t903: F, t2526: F, t9430: F, t9433: F, t9436: F, t9440: F, t9442: F, t9445: F, t9447: F, t9449: F, t9451: F) -> F {
    let t9454 = t8820 * t327 * t2562;
    let t9455 = t2560 * t9454;
    let t9457 = t2568 * t9454;
    let t9460 = t8820 * t291 * t7549;
    let t9461 = t7547 * t9460;
    let t9463 = t871 * t903;
    let t9464 = t9463 * t2526;
    let t9466 = -F::cast_from(0.27801896084645508334e-2_f64) * t9430 + F::cast_from(0.12163329537032409896e-2_f64) * t9433 - F::cast_from(0.42270452978984302532e-6_f64) * t9436 - F::cast_from(0.14480154210752868924e-5_f64) * t9440 + F::cast_from(0.17376185052903442709e-3_f64) * t9442 + F::cast_from(0.687148483626368822e-6_f64) * t9445 - F::cast_from(0.2318836277704281739e-4_f64) * t9447 + F::cast_from(0.16908181191593721013e-4_f64) * t9449 - F::cast_from(0.33816362383187442026e-4_f64) * t9451 + F::cast_from(0.1374296967252737644e-6_f64) * t9455 - F::cast_from(0.18326250058315256483e-6_f64) * t9457 - F::cast_from(0.45775879823985672486e-6_f64) * t9461 - F::cast_from(0.12357942809624928455e-3_f64) * t9464;
    t9466
}
