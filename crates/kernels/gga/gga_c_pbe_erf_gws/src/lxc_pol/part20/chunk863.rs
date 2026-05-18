//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 863/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk863<F: Float>(t2134: F, t8824: F, t1133: F, t874: F, t3179: F, t6331: F, t2146: F, t3165: F, t5: F, t2142: F, t3108: F, t3106: F, t4395: F) -> (F, F, F, F, F, F, F) {
    let t8826 = F::new(7.0) / F::new(144.0) * t2134 * t8824;
    let t8827 = t1133 * t874;
    let t8833 = t6331 * t3179;
    let t8835 = F::new(7.0) / F::new(72.0) * t2146 * t8833;
    let t8840 = t5 * t3165;
    let t8846 = F::new(7.0) / F::new(144.0) * t3108 * t2142;
    let t8847 = t4395 * t3106;
    (t8826, t8827, t8833, t8835, t8840, t8846, t8847)
}
