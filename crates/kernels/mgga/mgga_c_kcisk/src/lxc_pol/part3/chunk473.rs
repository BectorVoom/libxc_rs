//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 473/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk473<F: Float>(t3723: F, t3725: F, t1201: F, t1213: F, t3633: F, t3636: F, t3643: F, t3674: F, t3682: F, t3689: F, t3692: F, t3699: F, t3718: F, t45: F) -> (F, F) {
    let t3726 = t3723 * t3725;
    let t3729 = -t3633 + t3636 - t3643 + t3674 + t3682 + F::new(0.19751789702565206229e-1) * t45 * t3689 - F::new(0.11696446794910408142e1) * t3692 * t1213 + F::new(0.11696446794910408142e1) * t1201 * t3699 - F::new(0.58482233974552040708e0) * t1201 * t3718 - F::new(0.17315755899375863299e2) * t1201 * t3726;
    (t3726, t3729)
}
