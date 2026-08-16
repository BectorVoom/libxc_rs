//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 982/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk982<F: Float>(t20228: F, t20248: F, t20269: F, t20289: F, t20309: F, t20338: F, t20689: F, t20706: F, t1281: F, t6856: F, t1291: F, t6860: F) -> (F, F, F) {
    let t20709 = t20228 + t20248 + t20269 + t20289 + t20309 + t20338 + t20689 + t20706;
    let t20711 = t6856 * t1281;
    let t20721 = t6860 * t1291;
    (t20709, t20711, t20721)
}
