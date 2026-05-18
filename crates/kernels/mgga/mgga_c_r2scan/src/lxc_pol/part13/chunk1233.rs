//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1233/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1233<F: Float>(t1299: F, t3633: F, t11056: F, t2378: F, t2381: F, t37028: F, t37078: F, t1010: F, t1276: F, t11053: F, t8358: F, t19141: F, t3629: F) -> (F, F, F, F, F, F, F) {
    let t40770 = t3633 * t1299;
    let t40779 = t2378 * t11056;
    let t40781 = t37028 * t2381;
    let t40782 = F::new(4.0) / F::new(3.0) * t40781;
    let t40786 = F::new(44.0) / F::new(9.0) * t37078;
    let t40788 = t1276 * t11056 * t1010;
    let t40790 = t8358 * t11053;
    let t40792 = t19141 * t3629;
    (t40770, t40779, t40782, t40786, t40788, t40790, t40792)
}
