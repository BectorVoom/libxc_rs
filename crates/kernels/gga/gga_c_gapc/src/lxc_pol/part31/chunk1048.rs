//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1048/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1048<F: Float>(t12058: F, t1616: F, t11314: F, t11318: F, t11323: F, t11327: F, t11334: F, t11337: F, t11339: F, t11345: F, t11348: F, t11351: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12059 = t1616 * t12058;
    let t12060 = F::new(2.0) * t12059;
    let t12068 = F::cast_from(0.34752370105806885418e-3_f64) * t11314;
    let t12069 = F::cast_from(0.34752370105806885418e-3_f64) * t11318;
    let t12070 = F::cast_from(0.51491428373437201895e-5_f64) * t11323;
    let t12071 = F::cast_from(0.70344136651018351213e-8_f64) * t11327;
    let t12073 = F::cast_from(0.25340269868817520617e-3_f64) * t11334;
    let t12074 = F::cast_from(0.25301920572916666668e-5_f64) * t11337;
    let t12075 = F::cast_from(0.40483072916666666669e-4_f64) * t11339;
    let t12076 = F::cast_from(0.24458523220486111112e-4_f64) * t11345;
    let t12077 = F::cast_from(0.34752370105806885418e-3_f64) * t11348;
    let t12078 = F::cast_from(0.40483072916666666669e-4_f64) * t11351;
    (t12059, t12060, t12068, t12069, t12070, t12071, t12073, t12074, t12075, t12076, t12077, t12078)
}
