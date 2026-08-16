//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1234/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1234(t1066: f64, t154: f64, t18060: f64, t276: f64, t2057: f64, t2883: f64, t735: f64, t7620: f64, t1419: f64, t17874: f64, t17881: f64, t17890: f64, t2886: f64, t2891: f64, t7586: f64, t7594: f64, t7598: f64, t7602: f64, t7655: f64, t7660: f64, t7725: f64) -> f64 {
    let t21538 = t276 * t154 * t18060 * t1066;
    let t21540 = t2057 * t2883;
    let t21542 = t735 * t7620;
    let t21543 = t21542 / 54.0_f64;
    let t21559 = -5.0_f64 / 1296.0_f64 * t21538 - 11.0_f64 / 108.0_f64 * t21540 - t21543 - 5.0_f64 / 162.0_f64 * t17874 - t17881 + 11.0_f64 / 18.0_f64 * t1419 * t2886 * t2891 + t7586 * t7602 / 2.0_f64 - t7586 * t7594 / 3.0_f64 - t7586 * t7598 / 6.0_f64 + 0.68598428988911579154e-2_f64 * t7725 * t7655 + 0.34299214494455789577e-2_f64 * t7725 * t7660 + t17890 / 48.0_f64;
    t21559
}
