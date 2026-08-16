//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1129/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1129<F: Float>(t2381: F, t37028: F, t37078: F, t1010: F, t11056: F, t1276: F, t2391: F, t3366: F, t11050: F, t8358: F, t11885: F, t6654: F) -> (F, F, F, F, F, F) {
    let t40781 = t37028 * t2381;
    let t40786 = F::cast_from(44.0_f64) / F::cast_from(9.0_f64) * t37078;
    let t40788 = t1276 * t11056 * t1010;
    let t40797 = t1276 * t3366 * t2391;
    let t40804 = t8358 * t11050;
    let t40806 = t6654 * t11885;
    (t40781, t40786, t40788, t40797, t40804, t40806)
}
