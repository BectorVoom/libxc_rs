//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1128/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1128<F: Float>(t41499: F, t41502: F, t41505: F, t41508: F, t41516: F, t41519: F, t41522: F, t41525: F, t41528: F, t41531: F, t41540: F, t43631: F) -> F {
    let t43639 = -F::cast_from(0.17780800291358024693e0_f64) * t41499 + F::cast_from(0.88904001456790123462e-1_f64) * t41502 + F::cast_from(0.1333560021851851852e0_f64) * t41505 - F::cast_from(0.1333560021851851852e0_f64) * t41508 - t43631 + F::cast_from(0.16669500273148148149e-1_f64) * t41516 + F::cast_from(0.2469555596021947874e-1_f64) * t41519 - F::cast_from(0.22226000364197530866e-1_f64) * t41522 - F::cast_from(0.29634667152263374488e-1_f64) * t41525 + F::cast_from(0.69147556688614540471e-1_f64) * t41528 + F::cast_from(0.22226000364197530865e-1_f64) * t41531 + F::cast_from(0.17286889172153635117e0_f64) * t41540;
    t43639
}
