//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2003/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2003(t1877: f64, t23781: f64, t23796: f64, t23810: f64, t24191: f64, t24344: f64, t2522: f64, t25901: f64, t26744: f64, t26756: f64, t4314: f64, t6848: f64, t7110: f64, t7114: f64, t7656: f64, t7845: f64, t84791: f64, t89837: f64, t89840: f64, t89846: f64, t89872: f64, t89907: f64, t89931: f64, t89941: f64, t89982: f64, t89993: f64, t92276: f64) -> f64 {
    let t93246 = 3.0_f64 * t4314 * t7845 * t23781 + t26756 * t89872 + 2.0_f64 * t26756 * t89846 - 3.0_f64 / 2.0_f64 * t24191 * t89840 - t1877 * t26744 * t23810 - t1877 * t7114 * t89941 + t1877 * t24344 * t89982 - t1877 * t92276 * t6848 - t1877 * t7114 * t89907 / 2.0_f64 - 3.0_f64 * t24191 * t89931 - 3.0_f64 / 2.0_f64 * t24191 * t89837 + 3.0_f64 / 2.0_f64 * t2522 * t7845 * t23796 - 3.0_f64 * t24191 * t89993 - t1877 * t84791 * t7656 / 2.0_f64 + 3.0_f64 * t2522 * t7110 * t25901;
    t93246
}
