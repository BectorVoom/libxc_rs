//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1999/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1999(t26959: f64, t7428: f64, t27979: f64, t7032: f64, t1860: f64, t27956: f64, t7031: f64, t91890: f64, t91894: f64, t91896: f64, t91898: f64, t91900: f64, t91904: f64, t91905: f64, t91913: f64, t91921: f64) -> f64 {
    let t102137 = t7428 * t26959;
    let t102139 = t27979 * t7032;
    let t102142 = t1860 * t7031 * t27956;
    let t102145 = -16.0_f64 / 9.0_f64 * t102137 + 16.0_f64 / 9.0_f64 * t102139 - 8.0_f64 / 9.0_f64 * t102142 + t91890 + t91894 + t91896 + t91898 + t91900 + t91904 - 352.0_f64 / 27.0_f64 * t91905 + t91913 + t91921;
    t102145
}
