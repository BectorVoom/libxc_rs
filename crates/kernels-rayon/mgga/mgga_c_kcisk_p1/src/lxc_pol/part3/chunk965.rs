//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 965/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk965(t13294: f64, t14255: f64, t3484: f64, t5633: f64, t1163: f64, t4240: f64, t3482: f64, t3783: f64, t394: f64, t4210: f64, t1446: f64, t3908: f64, sigma0: f64) -> (f64, f64, f64, f64) {
    let t14256 = t14255 * t13294;
    let t14257 = t3484 * t14256;
    let t14258 = t5633 * t14257;
    let t14260 = t4240 * t1163;
    let t14261 = t3484 * t14260;
    let t14262 = t3482 * t14261;
    let t14264 = t3783 * sigma0;
    let t14265 = t14264 * t394;
    let t14266 = t4210 * t1163;
    let t14267 = t14265 * t14266;
    let t14268 = t3482 * t14267;
    let t14270 = t3908 * t1446;
    (t14258, t14262, t14268, t14270)
}
