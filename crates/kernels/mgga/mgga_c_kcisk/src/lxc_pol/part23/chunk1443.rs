//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1443/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1443<F: Float>(t22223: F, t2707: F, t111203: F, t111206: F, t111221: F, t111223: F, t1152: F, t1155: F, t115996: F, t115997: F, t22174: F, t2709: F, t2752: F, t294: F, t297: F, t32687: F, t33327: F, t33981: F, t33993: F, t3465: F, t6642: F, t9408: F, t9786: F, t9789: F) -> (F,) {
    let t116002 = t22223 * t2707;
    let t116010 = t111203 - t3465 * t9789 / 8.0 + t111206 + t1152 * t33981 / 8.0 - t2709 * t1155 * t6642 / 8.0 - t32687 * t9786 / 8.0 - t294 * t297 * (t115996 + t115997) / 16.0 + t116002 + t1152 * t33327 / 8.0 + t9408 * t33993 / 8.0 - t111221 - t111223 - t294 * t22174 * t2752 / 16.0;
    (t116010,)
}
