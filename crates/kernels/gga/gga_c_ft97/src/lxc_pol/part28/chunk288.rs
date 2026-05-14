//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 288/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk288<F: Float>(t1852: F, t3219: F, t83: F, t1882: F, t981: F, t432: F, t452: F, t986: F, t110: F, t3103: F, t499: F, t942: F, t487: F, t971: F) -> (F, F, F, F, F, F) {
    let t3220 = t1852 * t3219;
    let t3221 = t83 * t3220;
    let t3224 = t1882 * t981;
    let t3227 = t452 * t986 * t432;
    let t3231 = t452 * t110 * t3103;
    let t3235 = t452 * t499 * t942;
    let t3238 = t971 * t487;
    (t3221, t3224, t3227, t3231, t3235, t3238)
}
