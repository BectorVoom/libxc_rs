//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1386/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1386<F: Float>(t26282: F, t6094: F, t25670: F, t6086: F, t6093: F, t6518: F, t8210: F, t22731: F, t8157: F, t26249: F, t26251: F, t26255: F, t26259: F, t26262: F, t26265: F, t26268: F, t26271: F, t26276: F, t26279: F, t6452: F) -> (F,) {
    let t26283 = t26282 * t6094;
    let t26286 = t6093 * t6086 * t25670;
    let t26288 = t6518 * t8210;
    let t26290 = t22731 * t8157;
    let t26292 = 0.1713958891116262235e0 * t26249 - 0.15602799132097683414e1 * t26251 * t6452 - 0.20958572791407956061e0 * t26255 - t26259 - 0.87816964854445047168e-1 * t26262 + 0.87816964854445047166e-1 * t26265 - 0.1047928639570397803e0 * t26268 + 0.1047928639570397803e0 * t26271 + 0.58544643236296698111e-1 * t26276 + 0.34930954652346593433e-1 * t26279 + 0.1047928639570397803e0 * t26283 + 0.52396431978519890151e-1 * t26286 - 0.11524536070137145298e1 * t26288 + 0.1047928639570397803e0 * t26290;
    (t26292,)
}
