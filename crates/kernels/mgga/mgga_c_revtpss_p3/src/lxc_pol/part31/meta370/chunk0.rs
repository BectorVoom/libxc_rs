//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1403/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1403<F: Float>(t15008: F, t689: F, t213: F, t4469: F, t1580: F, t2440: F, t2439: F, t1569: F, t2453: F, t2458: F, t4321: F, t887: F) -> (F, F, F, F, F) {
    let t15010 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t15008;
    let t15011 = t213 * t4469;
    let t15014 = t2440 * t1580;
    let t15015 = t2439 * t15014;
    let t15017 = t2453 * t1569;
    let t15018 = t15017 * t2458;
    let t15045 = t4321 * t887;
    (t15010, t15011, t15015, t15018, t15045)
}
