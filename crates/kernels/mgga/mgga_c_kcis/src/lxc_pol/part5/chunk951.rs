//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 951/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk951<F: Float>(t9296: F, t9311: F, t783: F, t228: F, t2766: F, t2771: F, t2772: F, t2789: F, t8524: F, t899: F, t9005: F, t9007: F, t9010: F, t9017: F, t9018: F, t9021: F, t906: F, t9185: F, t9267: F, t9270: F, t9272: F, t9278: F, t9281: F) -> (F, F) {
    let t9312 = t9296 + t9311;
    let t9313 = t783 * t9312;
    let t9314 = t228 * t9005 - F::new(3.0) * t2766 * t2789 + F::new(6.0) * t2771 * t9021 + F::new(6.0) * t2772 * t9010 - t899 * t9185 - F::new(3.0) * t9007 * t906 - F::new(6.0) * t9017 * t9018 - t8524 - t9267 + t9270 + t9272 + t9278 - t9281 + t9313;
    (t9313, t9314)
}
