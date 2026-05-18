//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1207/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1207<F: Float>(t10687: F, t11479: F, t3275: F, t11514: F, t1554: F, t3579: F, t10831: F, t1102: F, t3692: F, t1543: F, t3582: F, t10610: F, t3276: F) -> (F, F, F, F) {
    let t40479 = t3275 * t11479 * t10687 / F::new(4.0);
    let t40483 = t3579 * t1554 * t11514 / F::new(4.0);
    let t40485 = t1102 * t10831 * t3692;
    let t40487 = t3582 * t1543;
    let t40490 = F::new(15.0) / F::new(8.0) * t10610 * t3276 * t40487;
    (t40479, t40483, t40485, t40490)
}
