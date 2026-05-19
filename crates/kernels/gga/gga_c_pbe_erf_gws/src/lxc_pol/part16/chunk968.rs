//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 968/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk968<F: Float>(t43: F, t50: F, t8079: F, t8082: F, t8084: F, t8086: F, t8088: F, t8091: F, t8094: F, t8096: F, t8098: F, t8100: F, zeta_threshold: F) -> F {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t8565 = piecewise3::<F>(t44, F::new(0.0), F::new(8.0) / F::new(27.0) * t8079 - F::new(8.0) / F::new(9.0) * t8082 - F::new(2.0) / F::new(9.0) * t8084 + F::new(4.0) / F::new(3.0) * t8086 - F::new(4.0) * t8088);
    let t8572 = piecewise3::<F>(t51, F::new(0.0), F::new(8.0) / F::new(27.0) * t8091 + F::new(8.0) / F::new(9.0) * t8094 - F::new(2.0) / F::new(9.0) * t8096 - F::new(4.0) / F::new(3.0) * t8098 + F::new(4.0) * t8100);
    let t8574 = t8565 / F::new(2.0) + t8572 / F::new(2.0);
    t8574
}
