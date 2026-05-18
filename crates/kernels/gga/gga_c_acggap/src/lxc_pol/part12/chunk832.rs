//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 832/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk832<F: Float>(t7755: F, t7756: F, t7759: F, t7761: F, t8252: F, t8253: F, t8254: F, t8257: F, t8904: F, t8909: F, t8913: F, t8917: F, t8921: F, t8925: F, t8930: F) -> F {
    let t9345 = F::new(0.10718504529517434243e-2) * t8904 + F::new(0.42874018118069736972e-3) * t8909 - F::new(0.21437009059034868486e-3) * t8913 - F::new(0.916875e-1) * t8917 - F::new(0.4584375e-1) * t8921 - F::new(0.4584375e-1) * t8925 - F::new(0.4584375e-1) * t8930 - t8252 - t8253 + t8254 + t8257 - t7755 + F::new(0.6431102717710460546e-2) * t7756 + t7759 - t7761;
    t9345
}
