//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 782/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk782<F: Float>(t1557: F, t5767: F, t1513: F, t4516: F, t103: F, t2: F, t39: F, t497: F, t542: F, t496: F, t1548: F, t156: F) -> (F, F, F, F, F, F) {
    let t5768 = t1557 * t5767;
    let t5770 = t1513 * t5767;
    let t5772 = param_hyb_omega_0 * t4516;
    let t5773 = t103 * t2;
    let t5776 = F::new(0.19486833333333333333e1) * t5772 * t5773 * t39;
    let t5783 = t542 * t497;
    let t5784 = t496 * t5783;
    let t5787 = t156 * t1548;
    (t5768, t5770, t5776, t5783, t5784, t5787)
}
