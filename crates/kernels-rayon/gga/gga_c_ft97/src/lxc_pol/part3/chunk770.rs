//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 770/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk770(t15959: f64, t7824: f64, t446: f64, t10993: f64, t11022: f64, t11024: f64, t11026: f64, t11070: f64, t11404: f64, t11417: f64, t11659: f64, t11781: f64, t15934: f64, t15938: f64, t15942: f64, t15945: f64, t15948: f64, t15953: f64, t15957: f64, t7775: f64, t8190: f64, t8192: f64) -> (f64, f64) {
    let t15960 = t7824 * t15959;
    let t15961 = t446 * t15960;
    let t15966 = t15934 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t15938 + t15942 / 27.0_f64 + 2.0_f64 / 9.0_f64 * t15945 - 5.0_f64 / 81.0_f64 * t15948 - t10993 - t11022 - t11024 + t11026 - 2.0_f64 / 9.0_f64 * t15953 + 2.0_f64 / 27.0_f64 * t15957 - 2.0_f64 / 9.0_f64 * t15961 - 2.0_f64 / 81.0_f64 * t7775 - 2.0_f64 / 27.0_f64 * t8192 - t11659 + t11070 - t11781 - t8190 + 2.0_f64 / 27.0_f64 * t11404 - t11417;
    (t15961, t15966)
}
