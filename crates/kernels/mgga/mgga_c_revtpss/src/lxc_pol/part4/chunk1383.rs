//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1383/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1383<F: Float>(t1263: F, t5245: F, t1122: F, t1042: F, t1234: F, t5390: F, t3704: F, t5293: F, t1121: F, t1214: F, t606: F, t1250: F) -> (F, F, F, F) {
    let t17500 = t1263 * t5245;
    let t17501 = t17500 * t1122;
    let t17502 = t1042 * t17501;
    let t17505 = t1234 * t5390;
    let t17509 = F::new(0.15244095330869239812e-2) * t5293 * t3704;
    let t17512 = t1214 * t1121;
    let t17513 = t17512 * t606;
    let t17514 = t1250 * t17513;
    (t17502, t17505, t17509, t17514)
}
