//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 964/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk964(t1506: f64, t4573: f64, t3119: f64, t4374: f64, t1111: f64, t12026: f64, t1509: f64, t1520: f64, t15225: f64, t15228: f64, t15255: f64, t15272: f64, t15327: f64, t15355: f64, t17663: f64, t17667: f64, t17670: f64, t17674: f64, t17677: f64, t3116: f64, t4363: f64, t4369: f64, t5314: f64, t5325: f64, t5337: f64) -> (f64, f64, f64) {
    let t17687 = t4573 * t1506;
    let t17688 = t17687 * t3119;
    let t17689 = t4374 * t17688;
    let t17696 = -t15225 / 54.0_f64 + t15228 / 288.0_f64 - 0.1420012659563261767e0_f64 * t3116 * t17663 + t1111 * t17667 / 48.0_f64 + 0.71000632978163088351e-1_f64 * t3116 * t17670 + 0.94667510637550784468e-1_f64 * t15255 - t1111 * t17674 / 48.0_f64 + t1111 * t17677 / 72.0_f64 + 0.35973654042269298099e1_f64 * t15355 * t1509 + 0.18352229811776266582e0_f64 * t15327 * t1520 - 0.91572784804598301689e1_f64 * t15272 - 0.75734008510040627576e0_f64 * t12026 * t5325 + 0.71000632978163088351e-1_f64 * t3116 * t17689 - 0.56800506382530470682e0_f64 * t4363 * t5314 + 0.57954409931925052365e-1_f64 * t4369 * t5337;
    (t17688, t17689, t17696)
}
