//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1138/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1138<F: Float>(t1184: F, t3195: F, t3295: F, t4039: F, t4022: F, t863: F, t6523: F, t8867: F, t1150: F, t14028: F, t14042: F, t14047: F, t14529: F, t14531: F, t14533: F, t14536: F, t14539: F) -> (F, F, F) {
    let t14542 = t1184 * t3195;
    let t14544 = t4039 * t3295;
    let t14547 = t863 * t4022;
    let t14548 = t6523 * t8867;
    let t14549 = t14547 * t14548;
    let t14551 = t14028 * t1150;
    let t14553 = -t14529 / F::new(768.0) - t14531 / F::new(192.0) - t14533 / F::new(48.0) - t14536 / F::new(48.0) - t14539 / F::new(96.0) + F::new(7.0) / F::new(144.0) * t14042 - t14542 / F::new(48.0) + t14544 / F::new(768.0) + F::new(7.0) / F::new(288.0) * t14047 + t14549 / F::new(16.0) - F::new(7.0) / F::new(1152.0) * t14551;
    (t14547, t14548, t14553)
}
