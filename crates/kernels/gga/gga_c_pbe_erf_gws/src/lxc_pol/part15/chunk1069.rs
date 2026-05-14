//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1069/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1069<F: Float>(t51945: F, t829: F, t830: F, t13949: F, t14001: F, t13957: F, t14113: F, t1195: F, t6729: F, t2222: F, t3955: F, t13953: F, t13976: F, t1176: F, t2298: F, t923: F) -> (F, F, F, F, F, F, F) {
    let t51947 = t829 * t830 * t51945;
    let t51952 = t14001 * t13949;
    let t51954 = t14113 * t13957;
    let t51957 = 455.0 / 1296.0 * t6729 * t1195;
    let t51958 = t3955 * t2222;
    let t51960 = t13953 * t13976;
    let t51963 = t1176 * t923 * t2298;
    (t51947, t51952, t51954, t51957, t51958, t51960, t51963)
}
