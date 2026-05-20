//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1642/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1642<F: Float>(t1042: F, t11285: F, t2866: F, t914: F, t936: F, t2869: F, t2919: F, t2923: F, t910: F) -> (F, F, F, F, F) {
    let t11286 = t1042 * t11285;
    let t11289 = t2866 * t914;
    let t11291 = F::new(3.0) * t11289 * t936;
    let t11293 = F::new(3.0) * t2869 * t2919;
    let t11294 = t910 * t2923;
    (t11286, t11289, t11291, t11293, t11294)
}
