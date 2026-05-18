//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1212/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1212<F: Float>(t13869: F, t13972: F, t13881: F, t840: F, t4052: F, t6781: F, t829: F, t830: F, t13949: F, t14001: F, t13957: F, t14113: F) -> (F, F, F, F, F) {
    let t51928 = t13972 * t13869;
    let t51930 = t840 * t13881;
    let t51945 = t6781 * t4052;
    let t51947 = t829 * t830 * t51945;
    let t51952 = t14001 * t13949;
    let t51954 = t14113 * t13957;
    (t51928, t51930, t51947, t51952, t51954)
}
