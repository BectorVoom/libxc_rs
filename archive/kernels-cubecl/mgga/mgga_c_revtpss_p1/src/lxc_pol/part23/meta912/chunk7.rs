//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2939/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2939<F: Float>(t63533: F, t63538: F, t63541: F, t63543: F, t63545: F, t63547: F, t63549: F, t63551: F, t77829: F, t77832: F, t77835: F, t77838: F) -> F {
    let t78075 = -F::cast_from(0.91285185185185185184e-1_f64) * t63533 + F::cast_from(0.5477111111111111111e0_f64) * t63538 - F::cast_from(0.98587999999999999998e0_f64) * t77829 + F::cast_from(0.49293999999999999999e0_f64) * t77832 - F::cast_from(0.82156666666666666668e-1_f64) * t77835 - F::cast_from(0.82156666666666666668e-1_f64) * t77838 - F::cast_from(0.32862666666666666666e0_f64) * t63541 + F::cast_from(0.5477111111111111111e-1_f64) * t63543 - F::cast_from(0.27385555555555555555e0_f64) * t63545 - F::cast_from(0.32862666666666666666e0_f64) * t63547 + F::cast_from(0.10954222222222222222e0_f64) * t63549 + F::cast_from(0.73028148148148148146e-1_f64) * t63551;
    t78075
}
