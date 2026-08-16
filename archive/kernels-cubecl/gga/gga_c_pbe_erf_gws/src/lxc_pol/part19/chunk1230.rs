//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1230/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1230<F: Float>(t14724: F, t343: F, t361: F, t14469: F, t50943: F, t13793: F, t53229: F, t3165: F, t898: F, t51509: F, t14456: F, t51666: F) -> (F, F, F, F, F, F) {
    let t53496 = t361 * t14724 * t343;
    let t53508 = t50943 * t14469;
    let t53515 = t53229 * t13793;
    let t53539 = t898 * t3165;
    let t53544 = F::cast_from(119.0_f64) / F::cast_from(6912.0_f64) * t51509;
    let t53545 = t51666 * t14456;
    (t53496, t53508, t53515, t53539, t53544, t53545)
}
