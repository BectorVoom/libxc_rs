//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1059/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1059(t11767: f64, t11770: f64, t11779: f64, t11782: f64, t11785: f64, t11787: f64, t11756: f64, t11762: f64, t11773: f64, t11776: f64, t12193: f64, t12194: f64, t12195: f64, t12196: f64, t12197: f64, t12198: f64, t12199: f64, t12200: f64, t12203: f64) -> f64 {
    let t12204 = 0.34752370105806885418e-3_f64 * t11767;
    let t12205 = 0.1422820120100248667e-7_f64 * t11770;
    let t12208 = 0.16908181191593721013e-5_f64 * t11779;
    let t12209 = 0.24760339692676868218e-5_f64 * t11782;
    let t12210 = 0.10551281119038438161e-7_f64 * t11785;
    let t12211 = 0.10551281119038438161e-7_f64 * t11787;
    let t12212 = t12193 + t12194 - t12195 + t12196 + t12197 - t12198 - t12199 + t12200 - 0.252977417353824213e-7_f64 * t11756 + 0.12228868272569444446e-4_f64 * t11762 - t12203 - t12204 + t12205 + 0.12650553385416666668e-5_f64 * t11773 + 0.12650553385416666668e-5_f64 * t11776 + t12208 + t12209 + t12210 + t12211;
    t12212
}
