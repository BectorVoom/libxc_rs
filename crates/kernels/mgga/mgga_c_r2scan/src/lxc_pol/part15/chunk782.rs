//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 782/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk782<F: Float>(t1343: F, t352: F, t1347: F, t349: F, t854: F, t2065: F, t2271: F, t2321: F, t607: F, t1783: F, t879: F, t2288: F, t6007: F) -> (F, F, F, F, F, F, F) {
    let t6751 = t352 * t1343;
    let t6755 = F::new(1.0) / t1347 / t349;
    let t6767 = F::new(1.0) / t1347 / t854;
    let t6794 = t2271 * t2065;
    let t6798 = t2321 * t607;
    let t6801 = t879 * t1783;
    let t6804 = t2288 * t6007;
    (t6751, t6755, t6767, t6794, t6798, t6801, t6804)
}
