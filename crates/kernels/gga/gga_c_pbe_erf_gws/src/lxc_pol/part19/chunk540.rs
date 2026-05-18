//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 540/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk540<F: Float>(t220: F, t34: F, t2735: F, t616: F, t1031: F, t202: F, t184: F) -> (F, F, F, F, F) {
    let t2736 = t220 * t34;
    let t2737 = t2735 * t2736;
    let t2739 = F::new(4.0) / F::new(15.0) * t616 * t2737;
    let t2740 = t202 * t1031;
    let t2741 = t2740 * t184;
    (t2736, t2737, t2739, t2740, t2741)
}
