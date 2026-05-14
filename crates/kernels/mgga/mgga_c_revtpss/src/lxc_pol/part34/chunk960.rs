//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 960/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk960<F: Float>(t5192: F, t6552: F, t1188: F, t24375: F, t3520: F, t1196: F, t1765: F, t20400: F, t5197: F, t6535: F, t6556: F, t12485: F, t3523: F, t1179: F, t24407: F, t1832: F, t6752: F) -> (F, F, F, F, F, F, F, F) {
    let t24478 = 0.17544670867903938621e1 * t5192 * t6552;
    let t24480 = t3520 * t24375 * t1188;
    let t24482 = 0.35089341735807877242e1 * t1196 * t24480;
    let t24484 = 0.17544670867903938621e1 * t20400 * t1765;
    let t24488 = t5197 * t6535;
    let t24490 = 0.35089341735807877242e1 * t1196 * t24488;
    let t24492 = 0.51947577317044391276e2 * t5192 * t6556;
    let t24493 = t12485 * t24375;
    let t24494 = t24493 * t3523;
    let t24496 = 0.10389515463408878255e3 * t1196 * t24494;
    let t24498 = t1179 * t24407 * t1188;
    let t24500 = 0.5848223622634646207e0 * t1196 * t24498;
    let t24501 = t6752 * t1832;
    (t24478, t24482, t24484, t24490, t24492, t24496, t24500, t24501)
}
