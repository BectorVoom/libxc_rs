//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 825/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk825<F: Float>(t1982: F, t7428: F, t8688: F, t2004: F, t9087: F, t2412: F, t7677: F, t2007: F, t2286: F, t7944: F, t1627: F, t2064: F, t3928: F, t25441: F, t8545: F, t1970: F, t236: F, t321: F, t3352: F, t5605: F) -> (F, F, F, F, F, F, F, F) {
    let t40505 = t8688 * t7428 * t1982;
    let t40507 = t9087 * t2004;
    let t40509 = t2412 * t7677;
    let t40511 = t9087 * t2007;
    let t40513 = t7944 * t2286;
    let t40516 = t3928 * t2064 * t1627;
    let t40518 = t25441 * t8545;
    let t40529 = t1970 * t3352 * t236 * t5605 * t321;
    (t40505, t40507, t40509, t40511, t40513, t40516, t40518, t40529)
}
