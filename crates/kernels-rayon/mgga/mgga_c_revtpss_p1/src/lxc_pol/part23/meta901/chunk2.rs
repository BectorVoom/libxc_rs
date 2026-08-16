//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2869/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2869(t231: f64, t2782: f64, t2783: f64, t76127: f64, t18615: f64, t18632: f64, t18677: f64, t2723: f64, t40314: f64, t40316: f64, t4494: f64, t4504: f64, t51396: f64, t51513: f64, t6022: f64, t62840: f64, t62843: f64, t62847: f64, t62853: f64, t820: f64) -> f64 {
    let t77197 = t2782 * t2783 * t76127 * t231;
    let t77213 = 0.54878743191129263322e-2_f64 * t77197 + 0.32927245914677557992e-1_f64 * t62840 + 0.11853808529283920877e2_f64 * t4504 * t18677 * t18632 + 0.21951497276451705328e-1_f64 * t62843 - t40314 + t40316 + t51513 - 0.19514881078765566037e-2_f64 * t62847 + 0.39512695097613069591e1_f64 * t4504 * t4494 * t2723 * t18615 + 0.39512695097613069591e1_f64 * t820 * t51396 * t6022 - 0.65854491829355115984e-1_f64 * t62853;
    t77213
}
