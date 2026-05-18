//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1254/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1254<F: Float>(t1144: F, t14191: F, t859: F, t14180: F, t4386: F, t14949: F, t9270: F, t53178: F, t53198: F, t53230: F, t53260: F, t53272: F) -> (F, F, F, F, F, F, F, F) {
    let t54978 = t859 * t1144 * t14191;
    let t54984 = t4386 * t1144 * t14180;
    let t54998 = F::new(7.0) / F::new(72.0) * t9270 * t14949;
    let t55005 = F::new(7.0) / F::new(288.0) * t53178;
    let t55007 = F::new(7.0) / F::new(288.0) * t53198;
    let t55022 = F::new(7.0) / F::new(72.0) * t53230;
    let t55031 = F::new(7.0) / F::new(72.0) * t53260;
    let t55036 = F::new(7.0) / F::new(72.0) * t53272;
    (t54978, t54984, t54998, t55005, t55007, t55022, t55031, t55036)
}
