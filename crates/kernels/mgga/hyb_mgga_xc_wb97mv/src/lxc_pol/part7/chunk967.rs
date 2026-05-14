//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 967/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk967<F: Float>(t9274: F, t7192: F, t7195: F, t7273: F, t9271: F, t9292: F, t939: F, t946: F, t2480: F, t3507: F, t941: F, t2486: F, t3502: F, t2492: F, t3513: F, t9353: F, t9357: F, t9360: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9366 = 2.0 / 3.0 * t9274;
    let t9367 = -t7273 + 8.0 / 9.0 * t7192 - t7195 / 3.0 + 4.0 / 9.0 * t9271 - t9366 + t9292;
    let t9368 = t939 * t9367;
    let t9370 = t946 * t9367;
    let t9372 = 0.60385e0 * t9274;
    let t9374 = t2480 * t3507;
    let t9375 = t9374 * t941;
    let t9377 = t3502 * t2486;
    let t9379 = t2492 * t3507;
    let t9380 = t9379 * t941;
    let t9382 = t3513 * t2486;
    let t9384 = 0.49671e0 * t9353 + 0.248355e0 * t9357 + 0.27595e0 * t9360 + 0.40256666666666666667e0 * t9271 + 0.258925e1 * t9368 + 0.16504875e0 * t9370 - t9372 + 0.905775e0 * t9292 - 0.258925e1 * t9375 - 0.1294625e1 * t9377 + 0.16504875e0 * t9380 + 0.82524375e-1 * t9382;
    (t9366, t9367, t9368, t9370, t9372, t9375, t9377, t9380, t9382, t9384)
}
