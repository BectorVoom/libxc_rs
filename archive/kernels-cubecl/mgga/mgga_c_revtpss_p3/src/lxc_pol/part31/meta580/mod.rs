//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta580 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1999;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2000;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta580<F: Float>(t11874: F, t27492: F, t11988: F, t7132: F, t11997: F, t25503: F, t3141: F, t1052: F, t3089: F, t1087: F, t11970: F, t1973: F, sigma0: F, t3201: F, t7126: F, t7114: F, t1024: F, t25576: F, t7120: F, t11858: F, t11926: F, t25516: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t93548, t93555, t93567, t93596, t93597, t93611) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1999::<F>(t11874, t27492, t11988, t7132, t11997, t25503, t3141, t1052, t3089, t1087, t11970, t1973, sigma0);
        let (t93618, t93622, t93646, t93655, t93658, t93667) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2000::<F>(t3201, t7126, t7114, t1024, t25576, t11997, t3141, t7120, t11858, t27492, t11926, t25516);
    (t93548, t93555, t93567, t93596, t93597, t93611, t93618, t93622, t93646, t93655, t93658, t93667)
}
