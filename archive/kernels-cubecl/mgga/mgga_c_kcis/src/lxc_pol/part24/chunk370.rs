//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 370/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk370<F: Float>(t1282: F, t187: F, t2184: F, t2190: F, t2201: F, t2205: F, t437: F, t119: F, t32: F, t5: F, t645: F, t88: F) -> (F, F, F) {
    let t2209 = t2184 - t2190 + t187 * (-t1282 * t2205 + t2201 * t437 - t2184 + t2190);
    let t2302 = F::cast_from(0.14764770444444444444e-2_f64) * t5 * t119 * t32;
    let t2303 = t88 * t645;
    (t2209, t2302, t2303)
}
