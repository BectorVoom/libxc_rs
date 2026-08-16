//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 962/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk962(t14223: f64, t1442: f64, t1452: f64, t3496: f64, t3739: f64, t3744: f64, t3748: f64, t3766: f64, t1286: f64, t3786: f64, t1450: f64, t3785: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14224 = t14223 * t1442;
    let t14226 = t14223 * t1452;
    let t14228 = t3739 * t3496;
    let t14230 = t3739 * t3744;
    let t14232 = t3748 * t3766;
    let t14234 = t3786 * t1286;
    let t14235 = t1450 * t14234;
    let t14236 = t3785 * t14235;
    (t14224, t14226, t14228, t14230, t14232, t14234, t14236)
}
