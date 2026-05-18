//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 515/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk515<F: Float>(t1328: F, t4158: F, t1220: F, t3729: F, t3774: F, t3780: F, t3789: F, t3793: F, t3801: F, t3807: F, t3808: F, t3810: F, t3910: F, t3917: F, t3920: F, t3925: F, t3930: F, t412: F) -> (F, F) {
    let t4159 = t4158 * t1328;
    let t4162 = F::new(0.22109259259259259258e-2) * t3774 - F::new(0.55273148148148148147e-3) * t3780 + F::new(0.49745833333333333332e-2) * t3789 + F::new(0.13265555555555555555e-1) * t3793 - F::new(0.33163888888888888888e-2) * t3801 + t3729 * t412 - t3807 - F::new(0.88437037037037037034e-2) * t3808 + F::new(0.33163888888888888888e-2) * t3810 + F::new(0.24872916666666666666e-2) * t3910 + F::new(0.24320185185185185185e-1) * t3917 - F::new(0.13265555555555555555e-1) * t3920 + F::new(0.193e0) * t1220 * t3925 + F::new(0.74498e-1) * t3930 * t3925 - F::new(0.193e0) * t1220 * t4159;
    (t4159, t4162)
}
