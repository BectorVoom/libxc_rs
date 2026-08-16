//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1041/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1041(t47784: f64, t42934: f64, t42937: f64, t42940: f64, t42943: f64, t42948: f64, t42954: f64, t42961: f64, t42963: f64, t42967: f64, t42970: f64, t47587: f64, t47594: f64, t47597: f64, t47602: f64, t47605: f64, t47607: f64, t47610: f64) -> (f64, f64) {
    let t50987 = 12.0_f64 * t47784;
    let t51000 = -0.17090058289204942852e-2_f64 * t47587 - t42934 - t42937 - t42940 + t42943 + t42948 - t42954 - t42961 + 0.7690526230142224284e-2_f64 * t42963 + 0.64087718584518535698e-3_f64 * t47594 - 0.3845263115071112142e-2_f64 * t42967 - 0.1281754371690370714e-2_f64 * t42970 - 0.64087718584518535698e-3_f64 * t47597 - t47602 + t47605 - 0.1922631557535556071e-2_f64 * t47607 + 0.1281754371690370714e-2_f64 * t47610;
    (t50987, t51000)
}
