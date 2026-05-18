//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1229/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1229<F: Float>(t4052: F, t8589: F, t829: F, t830: F, t13808: F, t14754: F, t14116: F, t3973: F, t1178: F, t904: F, t14688: F, t2397: F) -> (F, F, F, F, F) {
    let t52895 = t8589 * t4052;
    let t52897 = t829 * t830 * t52895;
    let t52901 = t13808 * t14754;
    let t52902 = F::new(7.0) / F::new(1152.0) * t52901;
    let t52906 = t3973 * t14116;
    let t52926 = t904 * t1178;
    let t52930 = t14688 * t2397;
    (t52897, t52902, t52906, t52926, t52930)
}
