//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1063/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1063<F: Float>(t332: F, t4408: F, t13869: F, t13972: F, t13949: F, t14001: F, t13957: F, t14113: F, t2222: F, t3955: F, t13953: F, t13976: F, t1176: F, t2298: F, t923: F, t13832: F) -> (F, F, F, F, F, F, F) {
    let t51922 = t4408 * t332;
    let t51928 = t13972 * t13869;
    let t51952 = t14001 * t13949;
    let t51954 = t14113 * t13957;
    let t51958 = t3955 * t2222;
    let t51960 = t13953 * t13976;
    let t51963 = t1176 * t923 * t2298;
    let t51964 = t51963 * t13832;
    (t51922, t51928, t51952, t51954, t51958, t51960, t51964)
}
