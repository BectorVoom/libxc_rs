//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 916/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk916<F: Float>(t8004: F, t4652: F, t4664: F, t4746: F, t4751: F, t4784: F, t4790: F, t4799: F, t7994: F, t7995: F, t7997: F, t7999: F, t8000: F, t8001: F, t8002: F, t8003: F) -> (F, F) {
    let t8005 = F::new(0.24415406715670879921e-3) * t8004;
    let t8006 = t4746 + t4751 + t4652 - t7994 - t7995 + t4664 + t7997 + t7999 - t4784 - t8000 - t4790 - t8001 - t8002 + t8003 + t8005 - t4799;
    (t8005, t8006)
}
