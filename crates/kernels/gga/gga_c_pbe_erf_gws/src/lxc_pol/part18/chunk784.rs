//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 784/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk784<F: Float>(t299: F, t481: F, t799: F, t5761: F, t4516: F, t103: F, t2: F, t39: F, t497: F, t542: F, t496: F, t120: F, t1508: F, t19: F, param_hyb_omega_0: F) -> (F, F, F, F, F, F) {
    let t5763 = t799 * t299 * t481;
    let t5764 = t5761 * t5763;
    let t5772 = param_hyb_omega_0 * t4516;
    let t5773 = t103 * t2;
    let t5776 = F::cast_from(0.19486833333333333333e1_f64) * t5772 * t5773 * t39;
    let t5783 = t542 * t497;
    let t5784 = t496 * t5783;
    let t5795 = t1508 * t120 * t19;
    (t5763, t5764, t5776, t5783, t5784, t5795)
}
