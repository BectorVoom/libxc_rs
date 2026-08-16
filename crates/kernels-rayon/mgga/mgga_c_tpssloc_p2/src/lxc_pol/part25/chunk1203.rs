//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1203/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1203(t531: f64, t7216: f64, t2056: f64, t40772: f64, t24334: f64, t2752: f64, t1877: f64, t2057: f64, t2249: f64, t22951: f64, t22964: f64, t24191: f64, t24335: f64, t24344: f64, t2522: f64, t26756: f64, t4314: f64, t6542: f64, t6671: f64, t7110: f64, t7114: f64, t81489: f64, t81492: f64, t81501: f64, t81505: f64, t81521: f64, t81529: f64, t81543: f64, t82313: f64, t82323: f64, t9257: f64) -> (f64, f64, f64, f64) {
    let t84733 = t531 * t7216;
    let t84766 = t2056 * t40772;
    let t84791 = t24334 * t2752;
    let t84795 = t1877 * t2057 * t9257 / 2.0_f64 - t1877 * t7114 * t82323 / 2.0_f64 + 9.0_f64 * t4314 * t2057 * t81543 + 3.0_f64 * t1877 * t24344 * t81521 + 3.0_f64 * t26756 * t81492 - 9.0_f64 / 2.0_f64 * t24191 * t81489 - 3.0_f64 * t1877 * t84766 * t82313 + 9.0_f64 * t4314 * t7110 * t22951 + 9.0_f64 / 2.0_f64 * t2522 * t24335 * t6542 + 9.0_f64 / 2.0_f64 * t2522 * t2057 * t81501 + 9.0_f64 / 2.0_f64 * t2522 * t2057 * t81505 - 3.0_f64 / 2.0_f64 * t1877 * t7114 * t81529 + 3.0_f64 / 2.0_f64 * t1877 * t7110 * t2249 + 9.0_f64 * t2522 * t7110 * t22964 - 3.0_f64 / 2.0_f64 * t1877 * t84791 * t6671;
    (t84733, t84766, t84791, t84795)
}
