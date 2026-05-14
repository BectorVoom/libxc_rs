//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1270/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1270<F: Float>(t1065: F, t32683: F, t32684: F, t32553: F, t32582: F, t32588: F, t32556: F, t33325: F, t33328: F, t33330: F, t2068: F, t32277: F, t5885: F, t13329: F, t1299: F, t2258: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t111533 = t1065 * t32683;
    let t111577 = 3.0 * t32684;
    let t111582 = 6.0 * t32553;
    let t111583 = 18.0 * t32582;
    let t111584 = 3.0 * t32588;
    let t111585 = 6.0 * t32556;
    let t113271 = t33325 / 8.0;
    let t113272 = t33328 / 8.0;
    let t113273 = 2.0 * t33330;
    let t113307 = t2068 * t32683;
    let t113350 = t5885 * t32277;
    let t113364 = t13329 * t32277;
    let t113369 = t2258 * t1299;
    (t111533, t111577, t111582, t111583, t111584, t111585, t113271, t113272, t113273, t113307, t113350, t113364, t113369)
}
