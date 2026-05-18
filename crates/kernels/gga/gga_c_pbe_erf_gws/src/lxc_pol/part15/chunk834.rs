//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 834/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk834<F: Float>(t2729: F, t586: F, t2609: F, t1037: F, t5467: F, t4913: F, t213: F, t331: F, t34: F, t649: F, t661: F, t1620: F) -> (F, F, F, F, F) {
    let t7011 = t2729 * t586;
    let t7013 = F::new(8.0) / F::new(15.0) * t7011 * t2609;
    let t7015 = F::new(8.0) / F::new(45.0) * t5467 * t1037;
    let t7017 = F::new(8.0) / F::new(15.0) * t4913 * t2609;
    let t7018 = t331 * t213;
    let t7019 = t649 * t34;
    let t7020 = t7019 * t661;
    let t7021 = t7018 * t7020;
    let t7023 = F::new(8.0) / F::new(15.0) * t1620 * t7021;
    (t7011, t7013, t7015, t7017, t7023)
}
