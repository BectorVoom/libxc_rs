//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1317/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1317<F: Float>(t23768: F, t28026: F, t28027: F, t795: F, t986: F, t1048: F, t31393: F, t19037: F, t19041: F, t19048: F, t19057: F, t23775: F, t31380: F, t31388: F, t32207: F, t19062: F) -> (F, F, F, F, F, F) {
    let t32208 = 0.31168546390226634765e3 * t23768;
    let t32209 = 3.0 * t28026;
    let t32210 = 0.17544670867903938621e1 * t28027;
    let t32212 = t986 * t795;
    let t32215 = 6.0 * t1048 * t31393 * t32212;
    let t32216 = -0.7089e1 * t31380 + t32207 - t19037 - t32208 + t19041 + t19048 - t23775 - t32209 + t32210 + t19057 - 0.14178e2 * t31388 - t32215;
    let t32217 = 0.51947577317044391277e2 * t19062;
    (t32208, t32209, t32210, t32215, t32216, t32217)
}
