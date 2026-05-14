//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 658/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk658<F: Float>(t1305: F, t2334: F, t1064: F, t2293: F, t599: F, t475: F, t2343: F, t1595: F, t876: F, t1324: F, t894: F, t1265: F, t2344: F, t555: F, t494: F, t2312: F, t2327: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6424 = t2334 * t1305;
    let t6425 = t1064 * t6424;
    let t6428 = t599 * t2293;
    let t6429 = t6428 * t475;
    let t6430 = t2343 * t6429;
    let t6433 = t1595 * t876;
    let t6438 = t894 * t1324;
    let t6443 = t2344 * t1265;
    let t6444 = t2343 * t6443;
    let t6447 = t555 * t2293;
    let t6448 = t6447 * t494;
    let t6451 = t2312 * t2327;
    (t6424, t6425, t6428, t6429, t6430, t6433, t6438, t6443, t6444, t6447, t6448, t6451)
}
