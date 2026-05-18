//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 647/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk647<F: Float>(t3553: F, t650: F, t186: F, t211: F, t1033: F, t1046: F, t1024: F, t2741: F, t3345: F, t220: F, t616: F, t3515: F, t3517: F, t3521: F, t3525: F, t3529: F, t3533: F, t3537: F, t3538: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3554 = t650 * t3553;
    let t3555 = t186 * t3554;
    let t3557 = F::new(2.0) / F::new(15.0) * t211 * t3555;
    let t3559 = F::new(4.0) / F::new(15.0) * t1033 * t1046;
    let t3561 = F::new(8.0) / F::new(15.0) * t2741 * t1024;
    let t3562 = -t3345;
    let t3563 = t220 * t3562;
    let t3564 = t186 * t3563;
    let t3566 = F::new(4.0) / F::new(15.0) * t616 * t3564;
    let t3567 = -t3515 + t3517 + t3521 + t3525 + t3529 + t3533 - t3537 + t3538 - t3557 - t3559 + t3561 + t3566;
    (t3554, t3555, t3557, t3559, t3561, t3562, t3563, t3564, t3566, t3567)
}
