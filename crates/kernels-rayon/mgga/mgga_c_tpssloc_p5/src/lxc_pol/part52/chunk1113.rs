//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1113/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1113(t27516: f64, t7364: f64, t5072: f64, t7376: f64, t7375: f64, t1215: f64, t1409: f64, t24851: f64, t24589: f64, t24812: f64, t24827: f64, t24849: f64, t27406: f64, t27481: f64, t27484: f64, t27492: f64, t27498: f64, t27502: f64, t27507: f64, t27511: f64, t7283: f64, t7368: f64, t7373: f64, t7378: f64) -> f64 {
    let t27517 = t27516 * t7364;
    let t27520 = t5072 * t7376;
    let t27521 = t7375 * t27520;
    let t27524 = t1409 * t1215;
    let t27525 = t27524 * t7376;
    let t27526 = t24851 * t27525;
    let t27529 = -0.82246703342411321825e-2_f64 * t7283 * t27481 - 0.82246703342411321825e-2_f64 * t7283 * t27484 + 0.27415567780803773942e-2_f64 * t24827 + 0.16449340668482264365e-1_f64 * t24812 * t27492 - 0.82246703342411321825e-2_f64 * t24812 * t27498 + 0.82246703342411321825e-2_f64 * t7373 * t27502 - 0.21932454224643019153e-1_f64 * t27507 * t7378 + 0.82246703342411321825e-2_f64 * t7373 * t27511 + 0.21932454224643019153e-1_f64 * t27406 * t7368 + 0.27415567780803773942e-2_f64 * t24589 * t27517 + 0.82246703342411321825e-2_f64 * t7373 * t27521 - 0.27415567780803773942e-2_f64 * t24849 * t27526;
    t27529
}
