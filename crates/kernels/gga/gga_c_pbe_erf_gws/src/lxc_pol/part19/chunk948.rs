//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 948/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk948<F: Float>(t3562: F, t649: F, t661: F, t1621: F, t1620: F, t2627: F, t7130: F, t1010: F, t7793: F, t2615: F, t2622: F, t3553: F) -> (F, F, F, F, F) {
    let t10691 = t649 * t3562;
    let t10692 = t10691 * t661;
    let t10693 = t1621 * t10692;
    let t10695 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1620 * t10693;
    let t10697 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t7130 * t2627;
    let t10699 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t7793 * t1010;
    let t10701 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t2615 * t2622;
    let t10702 = t649 * t3553;
    (t10695, t10697, t10699, t10701, t10702)
}
