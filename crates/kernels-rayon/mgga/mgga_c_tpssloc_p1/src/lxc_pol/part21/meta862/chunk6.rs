//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3135/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3135(t1174: f64, t6140: f64, t698: f64, t63841: f64, t63843: f64, t63845: f64, t63886: f64, t63888: f64, t63891: f64, t63893: f64, t63896: f64, t63899: f64, t63903: f64, t63906: f64, t63909: f64) -> (f64, f64) {
    let t64885 = t1174 * t698 * t6140;
    let t64903 = 8.0_f64 / 81.0_f64 * t63841 + 4.0_f64 / 9.0_f64 * t63843 - 2.0_f64 / 27.0_f64 * t63845 + 2.0_f64 / 9.0_f64 * t63886 + 5.0_f64 / 81.0_f64 * t63888 + t63891 / 9.0_f64 - 10.0_f64 / 27.0_f64 * t63893 - 2.0_f64 / 3.0_f64 * t63896 - 8.0_f64 / 27.0_f64 * t63899 - 2.0_f64 / 3.0_f64 * t63903 - t63906 / 3.0_f64 - t63909;
    (t64885, t64903)
}
