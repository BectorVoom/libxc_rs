//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 743/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk743<F: Float>(t871: F, t903: F, t2526: F, t9430: F, t9433: F, t9436: F, t9440: F, t9442: F, t9445: F, t9447: F, t9449: F, t9451: F, t9455: F, t9457: F, t9461: F, t2505: F, t904: F) -> (F, F) {
    let t9463 = t871 * t903;
    let t9464 = t9463 * t2526;
    let t9466 = -0.27801896084645508334e-2 * t9430 + 0.12163329537032409896e-2 * t9433 - 0.42270452978984302532e-6 * t9436 - 0.14480154210752868924e-5 * t9440 + 0.17376185052903442709e-3 * t9442 + 0.687148483626368822e-6 * t9445 - 0.2318836277704281739e-4 * t9447 + 0.16908181191593721013e-4 * t9449 - 0.33816362383187442026e-4 * t9451 + 0.1374296967252737644e-6 * t9455 - 0.18326250058315256483e-6 * t9457 - 0.45775879823985672486e-6 * t9461 - 0.12357942809624928455e-3 * t9464;
    let t9468 = t904 * t2505;
    (t9466, t9468)
}
