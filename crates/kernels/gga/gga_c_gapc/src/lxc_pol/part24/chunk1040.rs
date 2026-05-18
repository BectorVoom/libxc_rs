//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1040/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1040<F: Float>(t103: F, t2669: F, t2315: F, t2598: F, t2207: F, t640: F, t1645: F, t268: F, t2299: F, t830: F, t6856: F, t11925: F, t875: F) -> (F, F, F, F, F, F, F) {
    let t22442 = t2669 * t103;
    let t22581 = t2598 * t2315;
    let t22657 = t2207 * t640;
    let t22672 = t1645 * t268;
    let t22783 = t830 * t2299;
    let t22851 = t6856 * t103;
    let t22866 = t11925 * M_PI * t875;
    (t22442, t22581, t22657, t22672, t22783, t22851, t22866)
}
