//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 298/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk298<F: Float>(t106: F, t2405: F, t192: F, t524: F, t529: F, t901: F, t1457: F, t2335: F, t1564: F, t874: F, t475: F, t1445: F) -> (F, F, F, F, F) {
    let t2406 = t2405 * t106;
    let t2407 = t2406 * t192;
    let t2410 = t524 * t529;
    let t2411 = t2410 * t901;
    let t2413 = t1457 * t2335;
    let t2416 = t1564 * t874;
    let t2417 = t2416 * t475;
    let t2418 = t1445 * t2417;
    (t2407, t2411, t2413, t2416, t2418)
}
