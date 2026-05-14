//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 921/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk921<F: Float>(t12059: F, t11314: F, t11318: F, t11323: F, t11327: F, t11334: F, t11337: F, t11339: F, t11345: F, t11348: F, t11351: F, t11353: F, t11358: F, t11369: F, t11375: F, t11377: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12060 = 2.0 * t12059;
    let t12068 = 0.34752370105806885418e-3 * t11314;
    let t12069 = 0.34752370105806885418e-3 * t11318;
    let t12070 = 0.51491428373437201895e-5 * t11323;
    let t12071 = 0.70344136651018351213e-8 * t11327;
    let t12073 = 0.25340269868817520617e-3 * t11334;
    let t12074 = 0.25301920572916666668e-5 * t11337;
    let t12075 = 0.40483072916666666669e-4 * t11339;
    let t12076 = 0.24458523220486111112e-4 * t11345;
    let t12077 = 0.34752370105806885418e-3 * t11348;
    let t12078 = 0.40483072916666666669e-4 * t11351;
    let t12079 = 0.10821235962619981449e-3 * t11353;
    let t12080 = 0.42206481990611010728e-7 * t11358;
    let t12083 = 0.13259557375557346398e-6 * t11369;
    let t12086 = 0.21103240995305505364e-7 * t11375;
    let t12087 = 0.21103240995305505364e-7 * t11377;
    (t12060, t12068, t12069, t12070, t12071, t12073, t12074, t12075, t12076, t12077, t12078, t12079, t12080, t12083, t12086, t12087)
}
