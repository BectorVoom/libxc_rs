//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 373/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk373<F: Float>(t1629: F, t187: F, t2248: F, t2254: F, t2264: F, t2268: F, t633: F, t119: F, t32: F, t5: F, t645: F, t88: F) -> (F, F, F) {
    let t2272 = t2248 - t2254 + t187 * (-t1629 * t2268 + t2264 * t633 - t2248 + t2254);
    let t2302 = F::new(0.14764770444444444444e-2) * t5 * t119 * t32;
    let t2303 = t88 * t645;
    (t2272, t2302, t2303)
}
