//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 809/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk809<F: Float>(t7755: F, t7756: F, t7759: F, t7761: F, t8252: F, t8253: F, t8254: F, t8257: F, t8904: F, t8909: F, t8913: F, t8917: F, t8921: F, t8925: F, t8930: F) -> F {
    let t9345 = F::cast_from(0.10718504529517434243e-2_f64) * t8904 + F::cast_from(0.42874018118069736972e-3_f64) * t8909 - F::cast_from(0.21437009059034868486e-3_f64) * t8913 - F::cast_from(0.916875e-1_f64) * t8917 - F::cast_from(0.4584375e-1_f64) * t8921 - F::cast_from(0.4584375e-1_f64) * t8925 - F::cast_from(0.4584375e-1_f64) * t8930 - t8252 - t8253 + t8254 + t8257 - t7755 + F::cast_from(0.6431102717710460546e-2_f64) * t7756 + t7759 - t7761;
    t9345
}
