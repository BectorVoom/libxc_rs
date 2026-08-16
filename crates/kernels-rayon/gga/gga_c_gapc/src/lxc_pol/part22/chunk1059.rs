//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1059/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1059(t11863: f64, t11865: f64, t11867: f64, t11870: f64, t11873: f64, t11879: f64, t11890: f64, t11893: f64, t11895: f64, t11898: f64, t11900: f64, t11911: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12235 = 0.20240885416666666668e-4_f64 * t11863;
    let t12236 = 0.20240885416666666668e-4_f64 * t11865;
    let t12237 = 0.10821235962619981449e-3_f64 * t11867;
    let t12238 = 0.15387284965264388985e-8_f64 * t11870;
    let t12239 = 0.33764099580923002116e-6_f64 * t11873;
    let t12240 = 0.10110318318802209383e-5_f64 * t11879;
    let t12243 = 0.31675337336021900771e-5_f64 * t11890;
    let t12244 = 0.33764099580923002116e-6_f64 * t11893;
    let t12245 = 0.33764099580923002116e-6_f64 * t11895;
    let t12246 = 0.20010856351627032588e-7_f64 * t11898;
    let t12247 = 0.20047434126173032506e-6_f64 * t11900;
    let t12251 = 0.10551281119038438161e-7_f64 * t11911;
    (t12235, t12236, t12237, t12238, t12239, t12240, t12243, t12244, t12245, t12246, t12247, t12251)
}
