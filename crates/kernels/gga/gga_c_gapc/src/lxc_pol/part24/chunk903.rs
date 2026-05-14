//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 903/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk903<F: Float>(t2315: F, t286: F, t2801: F, t442: F, t8131: F, t2254: F, t8139: F, t186: F, t2153: F, t2206: F, t2389: F, t2211: F, t2299: F, t2404: F, t2546: F, t122: F, t188: F, t311: F, t6851: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18813 = t2315 * t286;
    let t18815 = t8131 * t2801 * t18813 * t442;
    let t18822 = t2254 * t286;
    let t18824 = t8139 * t18822 * t442;
    let t18856 = t2153 * t186;
    let t18866 = t2389 * t2206;
    let t19048 = t2211 * t2299;
    let t19055 = t2546 * t2404;
    let t19094 = t311 * t6851 * t122 * t188;
    (t18813, t18815, t18822, t18824, t18856, t18866, t19048, t19055, t19094)
}
