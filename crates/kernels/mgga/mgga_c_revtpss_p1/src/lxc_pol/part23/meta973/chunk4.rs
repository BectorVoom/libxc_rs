//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3302/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3302<F: Float>(t2782: F, t4086: F, t543: F, t86441: F, t22253: F, t47450: F, t47454: F, t47455: F, t49426: F, t49429: F, t49432: F, t5767: F, t75298: F, t75302: F, t75307: F, t820: F) -> F {
    let t86654 = t2782 * t4086 * t86441 * t543;
    let t86665 = F::cast_from(0.16463622957338778996e-1_f64) * t75298 - F::cast_from(0.32927245914677557992e-1_f64) * t75302 + F::cast_from(0.16463622957338778997e-1_f64) * t86654 - F::cast_from(0.19514881078765566038e-2_f64) * t49426 + F::cast_from(0.16463622957338778996e-1_f64) * t75307 + F::cast_from(0.19514881078765566038e-2_f64) * t49429 - F::cast_from(0.13878983423218070567e-1_f64) * t49432 - F::cast_from(0.46263278077393568556e-2_f64) * t47450 + t47454 - F::cast_from(0.26019841438354088051e-2_f64) * t47455 - F::cast_from(0.19756347548806534796e1_f64) * t820 * t5767 * t22253;
    t86665
}
