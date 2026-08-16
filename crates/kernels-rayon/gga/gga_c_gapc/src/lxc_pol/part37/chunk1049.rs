//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1049/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1049(t11493: f64, t11497: f64, t11501: f64, t11504: f64, t11506: f64, t11510: f64, t11524: f64, t11527: f64, t11529: f64, t11547: f64, t11552: f64, t11564: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12115 = 0.4637672555408563478e-4_f64 * t11493;
    let t12116 = 0.38647271295071362317e-6_f64 * t11497;
    let t12117 = 0.68714848362636882201e-6_f64 * t11501;
    let t12118 = 0.22510123728325872388e-7_f64 * t11504;
    let t12119 = 0.22510123728325872388e-6_f64 * t11506;
    let t12120 = 0.30353495895471971565e-6_f64 * t11510;
    let t12123 = 0.25301920572916666668e-5_f64 * t11524;
    let t12124 = 0.25301920572916666668e-5_f64 * t11527;
    let t12125 = 0.16217772716043213195e-2_f64 * t11529;
    let t12129 = 0.1422820120100248667e-7_f64 * t11547;
    let t12131 = 0.11594181388521408695e-4_f64 * t11552;
    let t12135 = 0.11594181388521408695e-4_f64 * t11564;
    (t12115, t12116, t12117, t12118, t12119, t12120, t12123, t12124, t12125, t12129, t12131, t12135)
}
