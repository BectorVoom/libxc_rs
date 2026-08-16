//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 490/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk490(t213: f64, t442: f64, t1390: f64, t967: f64, t1056: f64, t1399: f64, t970: f64, t1398: f64, t3583: f64, t1349: f64, t1391: f64, t173: f64, t3283: f64, t3844: f64, t3848: f64, t3851: f64, t3852: f64, t3853: f64) -> (f64, f64, f64) {
    let t3857 = t213 * t442;
    let t3858 = 0.15538616723388920628e-3_f64 * t3857;
    let t3859 = t967 * t1390;
    let t3860 = t3859 * t1056;
    let t3864 = t970 * t1399;
    let t3866 = t1398 * t3583;
    let t3869 = -t3844 - t3848 + t3851 - t3852 - 0.23911438650126355246e-1_f64 * t3853 + 0.11955719325063177623e-1_f64 * t1349 * t3283 + t3858 + 0.20718155631185227504e-3_f64 * t3860 - 0.5179538907796306876e-4_f64 * t1391 * t3283 - 0.23526125e-4_f64 * t3864 + 0.50413125e-5_f64 * t173 * t3866;
    (t3859, t3866, t3869)
}
