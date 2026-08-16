//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1015/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1015(t15629: f64, t504: f64, t69870: f64, t71516: f64, t75572: f64, t75590: f64, t75593: f64, t75596: f64, t75602: f64, t77664: f64, t77665: f64, t77666: f64, t77669: f64, t77670: f64, t77672: f64, t77677: f64, t77679: f64, t77681: f64) -> f64 {
    let t77682 = -0.2363e1_f64 * t71516 - 0.15372131649401827111e-4_f64 * t75572 - t77664 - t77665 + t77666 - 0.19957069503106347607e-1_f64 * t504 * t15629 + t77669 - t77670 - t77672 + 0.17347588262831798123e-4_f64 * t75590 + 0.17347588262831798123e-4_f64 * t75593 + 0.12263514265030957031e-4_f64 * t69870 - 0.81756761766873046877e-6_f64 * t75596 + t77677 + t75602 - t77679 - t77681;
    t77682
}
