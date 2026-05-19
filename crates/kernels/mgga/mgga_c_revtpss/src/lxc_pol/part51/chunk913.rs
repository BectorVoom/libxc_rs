//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 913/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk913<F: Float>(t31830: F, t8485: F, t817: F, t31809: F, t248: F, t822: F, t8479: F) -> (F, F, F, F, F) {
    let t31831 = t31830 * t8485;
    let t31832 = t31831 * t817;
    let t31833 = F::cast_from(0.33059535666846348619e-4_f64) * t31832;
    let t31834 = t31809 * t8485;
    let t31835 = t31834 * t248;
    let t31837 = t8479 * t822;
    (t31831, t31833, t31834, t31835, t31837)
}
