//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 911/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk911<F: Float>(t4652: F, t4664: F, t4744: F, t4746: F, t4751: F, t4784: F, t4790: F, t6076: F, t7985: F, t7987: F, t7989: F, t7991: F, t7992: F, t7994: F, t7995: F, t7997: F, t7999: F, t8000: F, t8001: F) -> (F,) {
    let t9044 = -t7985 + t7987 - t7989 + t7991 + t7992 + t4744 + t4746 + t4751 + t4652 - t7994 - t7995 + t4664 - t6076 + t7997 + t7999 - t4784 - t8000 - t4790 - t8001;
    (t9044,)
}
