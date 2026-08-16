//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2090/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2090(t1937: f64, t607: f64, t6722: f64, t10375: f64, t1942: f64, t1036: f64, t23551: f64, t23562: f64, t343: f64, t83032: f64, t210: f64, t23322: f64) -> (f64, f64, f64, f64, f64) {
    let t83075 = t6722 * t607 * t1937;
    let t83080 = t1942 * t10375 / 5184.0_f64;
    let t83082 = t23551 * t1036;
    let t83085 = t23562 * t83032 * t343;
    let t83092 = t23322 * t210;
    (t83075, t83080, t83082, t83085, t83092)
}
