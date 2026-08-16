//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1061/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1061(t11838: f64, t11843: f64, t11845: f64, t11851: f64, t11855: f64, t11826: f64, t11841: f64, t11858: f64, t12213: f64, t12214: f64, t12215: f64, t12216: f64, t12217: f64, t12218: f64, t12219: f64, t12220: f64, t12221: f64, t12222: f64, t12224: f64, t12225: f64) -> f64 {
    let t12226 = 0.11049275749843950004e-7_f64 * t11838;
    let t12228 = 0.11594181388521408695e-4_f64 * t11843;
    let t12229 = 0.11594181388521408695e-4_f64 * t11845;
    let t12230 = 0.28960308421505737848e-5_f64 * t11851;
    let t12231 = 0.25340269868817520617e-3_f64 * t11855;
    let t12233 = t12213 - t12214 - t12215 + t12216 - t12217 + t12218 + t12219 - t12220 + t12221 - t12222 + 0.53968515702149165443e-6_f64 * t11826 - t12224 + t12225 - t12226 - 0.57970906942607043474e-5_f64 * t11841 + t12228 - t12229 + t12230 - t12231 - 0.12650553385416666668e-5_f64 * t11858;
    t12233
}
