//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 517/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk517<F: Float>(t237: F, t6776: F, t1100: F, t6023: F, t6758: F, t6: F, t1113: F, t6027: F, t213: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t6777 = t237 * t6776;
    let t6778 = t1100 * t6777;
    let t6780 = t6023 * t6758;
    let t6783 = t237 * t6;
    let t6784 = t1100 * t6783;
    let t6785 = t6027 * t1113;
    let t6789 = t213 * sigma2;
    (t6777, t6778, t6780, t6783, t6784, t6785, t6789)
}
