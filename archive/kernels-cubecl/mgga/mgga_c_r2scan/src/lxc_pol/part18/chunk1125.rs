//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1125/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1125<F: Float>(t11894: F, t833: F, t1299: F, t3633: F, t11056: F, t2378: F, t2381: F, t37028: F, t1010: F, t1276: F, t2391: F, t3366: F) -> (F, F, F, F, F, F) {
    let t40764 = t11894 * t833;
    let t40770 = t3633 * t1299;
    let t40779 = t2378 * t11056;
    let t40781 = t37028 * t2381;
    let t40782 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t40781;
    let t40788 = t1276 * t11056 * t1010;
    let t40797 = t1276 * t3366 * t2391;
    (t40764, t40770, t40779, t40782, t40788, t40797)
}
