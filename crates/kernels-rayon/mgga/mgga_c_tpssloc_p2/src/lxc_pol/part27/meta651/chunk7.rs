//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2271/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2271(t1433: f64, t2307: f64, t72: f64, t26083: f64, t9231: f64, t2240: f64, t26043: f64, t33: f64, t12606: f64, t12648: f64, t12652: f64, t1409: f64, t14165: f64, t1860: f64, t1864: f64, t22489: f64, t22490: f64, t22502: f64, t22505: f64, t22513: f64, t22516: f64, t22537: f64, t26044: f64, t26045: f64, t26048: f64, t3961: f64, t3966: f64, t6486: f64, t6490: f64, t6492: f64, t6500: f64, t6509: f64, t67: f64, t7435: f64, t7441: f64, t7446: f64, t83788: f64, t83791: f64, t83796: f64, t83803: f64) -> f64 {
    let t90297 = t72 * t1433 * t2307;
    let t90308 = t9231 * t26083;
    let t90312 = t2240 * t33 * t26043;
    let t90315 = -t6486 * t26045 / 3.0_f64 - t6486 * t26048 / 3.0_f64 - t1860 * (220.0_f64 / 27.0_f64 * t83788 * t1409 - 40.0_f64 / 27.0_f64 * t83791 * t3961 - 40.0_f64 / 9.0_f64 * t22502 * t3966 - 5.0_f64 / 108.0_f64 * t83796 * t14165 + 5.0_f64 / 9.0_f64 * t22505 * t12652 + 5.0_f64 / 18.0_f64 * t22505 * t12648 + 5.0_f64 / 6.0_f64 * t6500 * t12606 + t83803) * t67 * t1864 / 6.0_f64 - t1860 * t26044 * t6509 / 3.0_f64 - t1860 * t7441 * t22489 / 6.0_f64 + 5.0_f64 / 6.0_f64 * t6490 * t90297 + t22537 * t7446 / 3.0_f64 + t7435 * t22513 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t7435 * t22516 + t7435 * t22490 / 3.0_f64 + 5.0_f64 / 3.0_f64 * t90308 * t6492 + 5.0_f64 / 3.0_f64 * t90312 * t6492;
    t90315
}
