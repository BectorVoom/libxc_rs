//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2793/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2793<F: Float>(t14939: F, t822: F, t686: F, t72: F, t874: F, t14574: F, t2439: F, t2777: F, t10943: F, t14502: F, t14507: F, t14546: F, t14547: F, t2724: F, t2754: F, t39588: F, t39620: F, t39624: F, t39629: F, t4494: F, t4504: F, t4514: F, t820: F, t837: F) -> F {
    let t51332 = t822 * t14939;
    let t51339 = t874 * t14939 * t72 * t686;
    let t51355 = t2439 * t2777 * t14574;
    let t51360 = -F::cast_from(0.19756347548806534796e1_f64) * t4514 * t14507 * t2754 - F::cast_from(0.19756347548806534796e1_f64) * t820 * t51332 * t837 - F::cast_from(0.33133632253434461091e-3_f64) * t39624 + F::cast_from(0.29272321618148349057e-1_f64) * t51339 - F::cast_from(0.19756347548806534796e1_f64) * t4514 * t4494 * t39620 + F::cast_from(0.11853808529283920877e2_f64) * t4504 * t4494 * t39588 + F::cast_from(0.58544643236296698114e-1_f64) * t39629 - F::cast_from(0.11853808529283920877e2_f64) * t14546 * t14507 * t14547 + F::cast_from(0.11853808529283920877e2_f64) * t4504 * t14507 * t2724 - F::cast_from(0.19514881078765566037e-2_f64) * t51355 + F::cast_from(0.39512695097613069591e1_f64) * t4504 * t14502 * t10943;
    t51360
}
