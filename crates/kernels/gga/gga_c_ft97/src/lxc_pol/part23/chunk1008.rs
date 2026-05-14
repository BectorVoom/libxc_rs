//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1008/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1008<F: Float>(t24412: F, t5064: F, t1168: F, t28023: F, t5147: F, t6154: F, t1449: F, t18391: F, t3977: F, t6940: F, t1091: F, t2354: F, t27787: F, t6118: F, t4973: F, t6119: F) -> (F, F, F, F, F, F, F, F) {
    let t30942 = t24412 * t5064;
    let t30946 = t28023 * t1168;
    let t30948 = t6154 * t5147;
    let t30950 = t18391 * t1449;
    let t30954 = t3977 * t6940;
    let t30959 = t2354 * t27787 * t1091;
    let t30960 = t6118 * t30959;
    let t30963 = t2354 * t6119 * t4973;
    (t30942, t30946, t30948, t30950, t30954, t30959, t30960, t30963)
}
