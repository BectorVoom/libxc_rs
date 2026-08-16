//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 230/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk230(t10: f64, t181: f64, t179: f64, t123: f64, t15: f64, t24: f64, t151: f64, t955: f64, t180: f64, t182: f64, t183: f64, t60: f64, t852: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t983 = t10 * t181;
    let t987 = t179 * t179;
    let t988 = t987 * t987;
    let t989 = t988 * t179;
    let t990 = t123 * t989;
    let t991 = t24 * t15;
    let t995 = t151 * t955;
    let t1001 = 0.13140859333333333333e-2_f64 * t180 * t983 * t183 - 0.98556444999999999995e-3_f64 * t990 * t991 * t183 - 0.19711288999999999999e-2_f64 * t180 * t182 * t995 - 4.0_f64 * t60 * t852;
    (t983, t989, t990, t991, t995, t1001)
}
