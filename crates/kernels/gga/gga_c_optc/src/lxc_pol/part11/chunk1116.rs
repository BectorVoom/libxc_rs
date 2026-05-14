//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1116/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1116<F: Float>(t151: F, t2124: F, t2126: F, t2168: F, t22787: F, t23040: F, t23270: F, t3467: F, t3501: F, t56082: F, t56103: F, t56106: F, t56107: F, t56115: F, t56119: F, t56127: F, t56128: F, t56135: F, t56136: F, t56144: F, t56145: F, t56149: F, t56404: F, t675: F, t9955: F, t9961: F) -> (F,) {
    let t56457 = 0.20863587575493018851e1 * t23040 * t675 * t56404 * t22787 - 0.10882232163006666614e1 * t3501 * t56082 - 0.417271751509860377e1 * t3467 * t2126 * t56106 - 0.31295381363239528276e1 * t9961 * t151 * t56135 + 0.72548214420044444093e1 * t2168 * t56145 - 0.21764464326013333228e1 * t3501 * t56107 - 0.10882232163006666614e1 * t9955 * t56136 + 0.24182738140014814697e0 * t2168 * t56115 - 0.90685268025055555116e-1 * t2168 * t56149 + 0.5441116081503333307e1 * t3501 * t56103 + 0.36274107210022222046e0 * t2168 * t56119 + 0.10431793787746509425e1 * t2124 * t2126 * t56127 + 0.36274107210022222046e0 * t2168 * t56128 + 0.83454350301972075403e1 * t2124 * t23270 * t56144;
    (t56457,)
}
