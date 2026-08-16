//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2793/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2793(t14939: f64, t822: f64, t686: f64, t72: f64, t874: f64, t14574: f64, t2439: f64, t2777: f64, t10943: f64, t14502: f64, t14507: f64, t14546: f64, t14547: f64, t2724: f64, t2754: f64, t39588: f64, t39620: f64, t39624: f64, t39629: f64, t4494: f64, t4504: f64, t4514: f64, t820: f64, t837: f64) -> f64 {
    let t51332 = t822 * t14939;
    let t51339 = t874 * t14939 * t72 * t686;
    let t51355 = t2439 * t2777 * t14574;
    let t51360 = -0.19756347548806534796e1_f64 * t4514 * t14507 * t2754 - 0.19756347548806534796e1_f64 * t820 * t51332 * t837 - 0.33133632253434461091e-3_f64 * t39624 + 0.29272321618148349057e-1_f64 * t51339 - 0.19756347548806534796e1_f64 * t4514 * t4494 * t39620 + 0.11853808529283920877e2_f64 * t4504 * t4494 * t39588 + 0.58544643236296698114e-1_f64 * t39629 - 0.11853808529283920877e2_f64 * t14546 * t14507 * t14547 + 0.11853808529283920877e2_f64 * t4504 * t14507 * t2724 - 0.19514881078765566037e-2_f64 * t51355 + 0.39512695097613069591e1_f64 * t4504 * t14502 * t10943;
    t51360
}
