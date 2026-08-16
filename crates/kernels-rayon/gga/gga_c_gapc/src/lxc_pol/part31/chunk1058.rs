//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1058/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1058(t11816: f64, t11818: f64, t11820: f64, t11823: f64, t11829: f64, t11832: f64, t11838: f64, t11843: f64, t11845: f64, t11851: f64, t11855: f64, t11863: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12219 = 0.12290803273518880209e-8_f64 * t11816;
    let t12220 = 0.32042899674547455013e-6_f64 * t11818;
    let t12221 = 0.11254699860307667372e-6_f64 * t11820;
    let t12222 = 0.30353495895471971565e-6_f64 * t11823;
    let t12224 = 0.12290803273518880209e-8_f64 * t11829;
    let t12225 = 0.8193868849012586806e-9_f64 * t11832;
    let t12226 = 0.11049275749843950004e-7_f64 * t11838;
    let t12228 = 0.11594181388521408695e-4_f64 * t11843;
    let t12229 = 0.11594181388521408695e-4_f64 * t11845;
    let t12230 = 0.28960308421505737848e-5_f64 * t11851;
    let t12231 = 0.25340269868817520617e-3_f64 * t11855;
    let t12235 = 0.20240885416666666668e-4_f64 * t11863;
    (t12219, t12220, t12221, t12222, t12224, t12225, t12226, t12228, t12229, t12230, t12231, t12235)
}
