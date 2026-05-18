//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 506/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk506<F: Float>(t179: F, t2068: F, t655: F, t299: F, t1843: F, t780: F, t291: F, t431: F, t3: F, t197: F, t290: F, t297: F) -> (F, F, F, F, F) {
    let t2070 = t179 * t2068 * t655;
    let t2071 = t299 * t2070;
    let t2074 = t179 * t780 * t1843;
    let t2077 = t291 * t431;
    let t2079 = F::new(1.0) / t3 / t2077;
    let t2082 = t290 * t197 * t2079 * t297;
    (t2070, t2071, t2074, t2079, t2082)
}
