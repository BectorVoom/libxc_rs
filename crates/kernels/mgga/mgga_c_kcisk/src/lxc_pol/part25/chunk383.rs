//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 383/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk383<F: Float>(t1791: F, t2469: F, t1801: F, t2364: F, t1800: F, t1799: F, t2063: F, t682: F) -> (F, F, F, F, F) {
    let t2470 = t2469 * t1791;
    let t2473 = t1801 * t2364;
    let t2474 = t1800 * t2473;
    let t2475 = t1799 * t2474;
    let t2477 = t682 * t2063;
    (t2470, t2473, t2474, t2475, t2477)
}
